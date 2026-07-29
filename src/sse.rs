//! Server-Sent Events parsing and a reconnecting session event stream.
//!
//! This is the one place the SDK streams rather than buffers. [`HttpClient`]
//! reads a whole response body before returning, which is correct for every
//! JSON route but cannot express a connection that stays open indefinitely, so
//! the session stream drives `reqwest` directly.
//!
//! The backend emits frames of the form:
//!
//! ```text
//! id: 42
//! data: {"seq":42,"at":...,"sessionId":"...","event":{"kind":"assistant","body":"hi"}}
//!
//! : ping
//!
//! ```
//!
//! `id:` sets the replay cursor (persisted events only; streaming deltas omit
//! it), `data:` carries the JSON [`EventEnvelope`], comment lines (`: ping`) are
//! ignored, and a blank line terminates the current frame.

use std::collections::VecDeque;

use futures::stream::{BoxStream, Stream, StreamExt};

use crate::api::medulla_types::EventEnvelope;
use crate::{enc, Error, HttpClient};

/// A completed SSE frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseFrame {
    /// Cursor value from an `id:` line, when present.
    pub id: Option<u64>,
    /// Concatenated `data:` payload (lines joined with `\n`).
    pub data: String,
}

/// Incremental SSE line parser. Feed byte chunks; collect completed frames.
#[derive(Debug, Default)]
pub struct SseParser {
    /// Bytes of an incomplete trailing line.
    line_buf: String,
    /// Accumulated `data:` payload for the in-progress frame.
    data: String,
    /// Whether any `data:` line has been seen for the in-progress frame.
    got_data: bool,
    /// `id:` value seen for the in-progress frame.
    id: Option<u64>,
}

impl SseParser {
    /// Create an empty parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed a chunk of decoded text, appending any completed frames to `out`.
    ///
    /// Chunk boundaries are arbitrary: a frame may span several chunks and a
    /// chunk may contain several frames.
    pub fn feed(&mut self, chunk: &str, out: &mut Vec<SseFrame>) {
        self.line_buf.push_str(chunk);
        while let Some(newline) = self.line_buf.find('\n') {
            let mut line = self.line_buf[..newline].to_string();
            self.line_buf.drain(..=newline);
            if line.ends_with('\r') {
                line.pop();
            }
            self.feed_line(&line, out);
        }
    }

    fn feed_line(&mut self, line: &str, out: &mut Vec<SseFrame>) {
        if line.is_empty() {
            // A blank line terminates the frame.
            if self.got_data || self.id.is_some() {
                out.push(SseFrame {
                    id: self.id.take(),
                    data: std::mem::take(&mut self.data),
                });
            }
            self.got_data = false;
            self.id = None;
            return;
        }
        // Comment line (`: ...`, e.g. `: ping`) — ignore.
        if line.starts_with(':') {
            return;
        }
        let (field, value) = match line.find(':') {
            Some(i) => {
                let value = &line[i + 1..];
                // A single leading space after the colon is stripped.
                (&line[..i], value.strip_prefix(' ').unwrap_or(value))
            }
            None => (line, ""),
        };
        match field {
            "id" => {
                if let Ok(seq) = value.trim().parse::<u64>() {
                    self.id = Some(seq);
                }
            }
            "data" => {
                if self.got_data {
                    self.data.push('\n');
                }
                self.data.push_str(value);
                self.got_data = true;
            }
            // `event:`, `retry:` and unknown fields are not used here.
            _ => {}
        }
    }
}

/// Seq-based de-duplication for reconnect replay.
///
/// Frames carrying a persisted `seq` are only accepted when they advance past
/// the cursor; frames without a seq (streaming deltas) always pass.
#[derive(Debug, Default)]
pub struct SeqDedup {
    cursor: Option<u64>,
}

impl SeqDedup {
    /// Start from an optional last-seen seq (the reconnect `Last-Event-ID`).
    pub fn new(start: Option<u64>) -> Self {
        Self { cursor: start }
    }

    /// The current cursor, suitable for a `Last-Event-ID` reconnect header.
    pub fn cursor(&self) -> Option<u64> {
        self.cursor
    }

    /// Decide whether a frame with the given seq should be yielded, advancing
    /// the cursor when it does.
    pub fn accept(&mut self, seq: Option<u64>) -> bool {
        match seq {
            None => true,
            Some(seq) => {
                if self.cursor.is_none_or(|cursor| seq > cursor) {
                    self.cursor = Some(seq);
                    true
                } else {
                    false
                }
            }
        }
    }
}

/// Internal driver state for the reconnecting stream.
struct StreamState {
    http: reqwest::Client,
    url: String,
    parser: SseParser,
    dedup: SeqDedup,
    pending: VecDeque<Result<EventEnvelope, Error>>,
    body: Option<BoxStream<'static, reqwest::Result<Vec<u8>>>>,
    first_connect: bool,
}

impl StreamState {
    /// Open (or reopen) the SSE connection using the current cursor.
    async fn connect(&mut self) -> Result<(), Error> {
        if !self.first_connect {
            // Small backoff between reconnect attempts.
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        self.first_connect = false;
        let mut request = self
            .http
            .get(&self.url)
            .header(reqwest::header::ACCEPT, "text/event-stream");
        if let Some(cursor) = self.dedup.cursor() {
            request = request.header("Last-Event-ID", cursor.to_string());
        }
        let response = request.send().await?.error_for_status()?;
        // Map chunks to owned bytes so the stored stream item type stays
        // nameable without depending on `bytes` directly.
        let body = response
            .bytes_stream()
            .map(|chunk| chunk.map(|b| b.to_vec()));
        self.body = Some(body.boxed());
        Ok(())
    }

    /// Convert a completed frame into a deduped, decoded envelope (if any).
    fn ingest(&mut self, frame: SseFrame) {
        if !self.dedup.accept(frame.id) {
            return;
        }
        let trimmed = frame.data.trim();
        if trimmed.is_empty() {
            return;
        }
        match serde_json::from_str::<EventEnvelope>(trimmed) {
            Ok(envelope) => self.pending.push_back(Ok(envelope)),
            Err(e) => self.pending.push_back(Err(Error::Decode(e))),
        }
    }

    /// Produce the next stream item, reconnecting as needed.
    async fn next(&mut self) -> Option<Result<EventEnvelope, Error>> {
        loop {
            if let Some(item) = self.pending.pop_front() {
                return Some(item);
            }
            if self.body.is_none() {
                if let Err(e) = self.connect().await {
                    // Surface the connect error, then retry on the next poll.
                    return Some(Err(e));
                }
            }
            let body = self.body.as_mut().expect("body set above");
            match body.next().await {
                Some(Ok(bytes)) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let mut frames = Vec::new();
                    self.parser.feed(&text, &mut frames);
                    for frame in frames {
                        self.ingest(frame);
                    }
                }
                Some(Err(e)) => {
                    self.body = None;
                    return Some(Err(Error::Http(e)));
                }
                None => {
                    // Server closed the connection; reconnect from the cursor.
                    self.body = None;
                }
            }
        }
    }
}

/// Build a reconnecting SSE stream of [`EventEnvelope`]s.
///
/// `url` must already carry authentication (`?token=<jwt>`), because
/// `EventSource`-style consumers cannot set headers on the initial request. The
/// stream reconnects with `Last-Event-ID` and de-duplicates replayed frames by
/// seq. Drop the returned stream to stop.
pub fn event_stream(
    http: reqwest::Client,
    url: String,
    last_event_id: Option<u64>,
) -> impl Stream<Item = Result<EventEnvelope, Error>> {
    let state = StreamState {
        http,
        url,
        parser: SseParser::new(),
        dedup: SeqDedup::new(last_event_id),
        pending: VecDeque::new(),
        body: None,
        first_connect: true,
    };
    futures::stream::unfold(state, |mut state| async move {
        state.next().await.map(|item| (item, state))
    })
}

impl HttpClient {
    /// Open a reconnecting event stream for one Medulla session
    /// (`GET /medulla/v1/sessions/{id}/stream`).
    ///
    /// The bearer token is passed as a query parameter rather than a header:
    /// the stream is reconnected by this client and by browser `EventSource`
    /// consumers alike, and the latter cannot set one.
    pub fn session_event_stream(
        &self,
        session_id: &str,
        last_event_id: Option<u64>,
    ) -> impl Stream<Item = Result<EventEnvelope, Error>> {
        let mut url = format!(
            "{}/medulla/v1/sessions/{}/stream",
            self.base_url,
            enc(session_id)
        );
        if let Some(token) = &self.token {
            url.push_str(&format!("?token={}", enc(token)));
        }
        event_stream(self.client.clone(), url, last_event_id)
    }
}

#[cfg(test)]
mod tests;
