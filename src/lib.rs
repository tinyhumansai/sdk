//! Rust SDK for the TinyHumans backend.
//!
//! [`TinyHumansClient`] exposes one typed namespace accessor per backend area,
//! each with one method per deployed operation. [`TinyHumansClient::raw`] is the
//! escape hatch for routes not yet surfaced as typed methods.

use generated_public_routes::{PUBLIC_ROUTES, UNEXPOSED_ROUTES};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as ReqwestClient, Method};
use serde_json::Value;
use url::Url;

pub mod api;
pub mod generated_public_routes;
pub mod jwt;
pub mod socket;
pub mod sse;

/// Bytes left un-encoded by `encodeURIComponent`: the unreserved set
/// `A-Z a-z 0-9 - _ . ! ~ * ' ( )`.
const COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');

/// Percent-encode a single path segment (parity with `encodeURIComponent`).
pub fn enc(value: &str) -> String {
    utf8_percent_encode(value, COMPONENT).to_string()
}

/// A query parameter pair. `None` values are skipped when the request is built.
pub type QueryParam = (&'static str, Option<String>);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    #[error("http client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("http {status}: {body}")]
    Status { status: u16, body: Value },
    #[error("invalid header value: {0}")]
    Header(#[from] reqwest::header::InvalidHeaderValue),
    #[error("response decoding failed: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("socket.io transport failed: {0}")]
    Socket(Box<rust_socketio::Error>),
    #[error("a bearer token is required for socket.io connections")]
    MissingSocketToken,
    #[error("socket event {0} did not carry a JSON payload")]
    UnexpectedSocketPayload(String),
    #[error("socket acknowledgement timed out")]
    SocketAckTimeout,
    #[error("socket acknowledgement channel closed")]
    SocketAckClosed,
    #[error("route is intentionally not exposed by the SDK: {0} {1}")]
    RouteNotExposed(String, String),
    /// The response carried a `{success:false, ...}` envelope.
    ///
    /// The backend does not always pair an unsuccessful envelope with a
    /// non-2xx status, so this is distinct from [`Error::Status`]: the
    /// transport succeeded and the operation did not.
    #[error("backend reported failure: {error}")]
    Envelope {
        error: String,
        error_code: Option<String>,
        details: Value,
    },
}

impl From<rust_socketio::Error> for Error {
    fn from(error: rust_socketio::Error) -> Self {
        Self::Socket(Box::new(error))
    }
}

#[derive(Clone)]
pub struct TinyHumansClient {
    http: HttpClient,
}

impl TinyHumansClient {
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            http: HttpClient::new(base_url.as_ref()),
        }
    }

    pub fn with_token(mut self, token: Option<String>) -> Self {
        self.http.token = token;
        self
    }

    pub fn with_api_key(mut self, api_key: Option<String>) -> Self {
        self.http.api_key = api_key;
        self
    }

    /// Use a caller-supplied [`reqwest::Client`] instead of the crate default.
    ///
    /// The default client is `reqwest::Client::new()`: bundled rustls roots, no
    /// timeout, no proxy configuration. That is a reasonable default for a
    /// standalone script but wrong for an embedded host, which needs the SDK's
    /// requests to share its own transport policy — TLS backend, timeouts,
    /// proxy, redirect policy, connection pool.
    ///
    /// The concrete motivating case is TLS backend selection. On Windows a
    /// corporate TLS-inspection proxy presents a certificate chained to a root
    /// that is in the OS certificate store but not in the bundled rustls root
    /// set, so every request fails until the client is built on schannel.
    /// A host that already resolves this per platform passes that client here.
    pub fn with_http_client(mut self, client: ReqwestClient) -> Self {
        self.http.client = client;
        self
    }

    /// Attach headers sent on every request in addition to the SDK's own.
    ///
    /// For host build/version attribution and similar cross-cutting metadata
    /// that would otherwise have to be threaded through each call. The SDK's
    /// own headers (`accept`, `content-type`, `x-sdk-client`) and the
    /// credential headers (`authorization`, `x-api-key`) are applied after
    /// these and therefore win on conflict — a caller cannot accidentally
    /// unset the bearer token or misreport the SDK client identity.
    pub fn with_default_headers(mut self, headers: HeaderMap) -> Self {
        self.http.default_headers = headers;
        self
    }

    /// Raw HTTP escape hatch for routes without a typed method yet.
    pub fn raw(&self) -> &HttpClient {
        &self.http
    }

    pub fn agent_integrations(&self) -> api::agent_integrations::AgentIntegrationsApi<'_> {
        api::agent_integrations::AgentIntegrationsApi::new(&self.http)
    }
    pub fn announcements(&self) -> api::announcements::AnnouncementsApi<'_> {
        api::announcements::AnnouncementsApi::new(&self.http)
    }
    pub fn auth(&self) -> api::auth::AuthApi<'_> {
        api::auth::AuthApi::new(&self.http)
    }
    pub fn channels(&self) -> api::channels::ChannelsApi<'_> {
        api::channels::ChannelsApi::new(&self.http)
    }
    pub fn coupons(&self) -> api::coupons::CouponsApi<'_> {
        api::coupons::CouponsApi::new(&self.http)
    }
    pub fn feedback(&self) -> api::feedback::FeedbackApi<'_> {
        api::feedback::FeedbackApi::new(&self.http)
    }
    pub fn health(&self) -> api::health::HealthApi<'_> {
        api::health::HealthApi::new(&self.http)
    }
    pub fn inference(&self) -> api::inference::InferenceApi<'_> {
        api::inference::InferenceApi::new(&self.http)
    }
    pub fn api_keys(&self) -> api::api_keys::ApiKeysApi<'_> {
        api::api_keys::ApiKeysApi::new(&self.http)
    }
    pub fn budgets(&self) -> api::budgets::BudgetsApi<'_> {
        api::budgets::BudgetsApi::new(&self.http)
    }
    pub fn invite(&self) -> api::invite::InviteApi<'_> {
        api::invite::InviteApi::new(&self.http)
    }
    pub fn mascots(&self) -> api::mascots::MascotsApi<'_> {
        api::mascots::MascotsApi::new(&self.http)
    }
    pub fn medulla(&self) -> api::medulla::MedullaApi<'_> {
        api::medulla::MedullaApi::new(&self.http)
    }
    pub fn opencompany(&self) -> api::opencompany::OpenCompanyApi<'_> {
        api::opencompany::OpenCompanyApi::new(&self.http)
    }
    pub fn orchestration(&self) -> api::orchestration::OrchestrationApi<'_> {
        api::orchestration::OrchestrationApi::new(&self.http)
    }
    pub fn payments(&self) -> api::payments::PaymentsApi<'_> {
        api::payments::PaymentsApi::new(&self.http)
    }
    pub fn redirect(&self) -> api::redirect::RedirectApi<'_> {
        api::redirect::RedirectApi::new(&self.http)
    }
    pub fn referral(&self) -> api::referral::ReferralApi<'_> {
        api::referral::ReferralApi::new(&self.http)
    }
    pub fn rewards(&self) -> api::rewards::RewardsApi<'_> {
        api::rewards::RewardsApi::new(&self.http)
    }
    pub fn teams(&self) -> api::teams::TeamsApi<'_> {
        api::teams::TeamsApi::new(&self.http)
    }
    pub fn webhooks(&self) -> api::webhooks::WebhooksApi<'_> {
        api::webhooks::WebhooksApi::new(&self.http)
    }

    /// Connect to the authenticated Socket.IO surface at `/socket.io/`.
    ///
    /// The returned connection receives every public socket event through one
    /// generic stream and also exposes typed helpers for the Medulla harness
    /// and workflow protocol.
    pub async fn connect_socket(&self) -> Result<socket::SocketConnection, Error> {
        let token = self.http.token.clone().ok_or(Error::MissingSocketToken)?;
        socket::SocketConnection::connect(self.http.base_url.clone(), token).await
    }
}

#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
    token: Option<String>,
    api_key: Option<String>,
    client: ReqwestClient,
    default_headers: HeaderMap,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            token: None,
            api_key: None,
            client: ReqwestClient::new(),
            default_headers: HeaderMap::new(),
        }
    }

    /// Core request primitive used by every typed namespace method.
    ///
    /// - `query` pairs with a `None` value are omitted.
    /// - `body` is sent as JSON when present.
    /// - `unwrap` controls whether a `{success,data}` envelope is unwrapped.
    pub async fn send(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: Option<&Value>,
        unwrap: bool,
    ) -> Result<Value, Error> {
        reject_unexposed_route(&method, path)?;
        let url = self.url(path, query)?;
        let mut request = self.client.request(method, url).headers(self.headers()?);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        let value = if text.is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text).unwrap_or(Value::String(text))
        };
        if !status.is_success() {
            return Err(Error::Status {
                status: status.as_u16(),
                body: value,
            });
        }
        if unwrap {
            unwrap_envelope(value)
        } else {
            Ok(value)
        }
    }

    /// Send a request and deserialize the unwrapped response into a concrete DTO.
    pub async fn send_typed<T: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        query: &[QueryParam],
        body: Option<&Value>,
        unwrap: bool,
    ) -> Result<T, Error> {
        let value = self.send(method, path, query, body, unwrap).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Convenience GET on the raw client (unwraps the envelope).
    pub async fn get(&self, path: &str) -> Result<Value, Error> {
        self.send(Method::GET, path, &[], None, true).await
    }

    /// Convenience POST on the raw client (unwraps the envelope).
    pub async fn post(&self, path: &str, body: &Value) -> Result<Value, Error> {
        self.send(Method::POST, path, &[], Some(body), true).await
    }

    pub async fn post_multipart(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
    ) -> Result<Value, Error> {
        reject_unexposed_route(&Method::POST, path)?;
        let url = self.url(path, &[])?;
        // A multipart request's `Content-Type` (`multipart/form-data; boundary=…`)
        // is owned by reqwest's `.multipart(form)`. Our shared `headers()` sets a
        // fixed `application/json`; left in place it wins over the multipart type,
        // so the backend JSON-parses the multipart body and 500s with
        // `Unexpected token '-', "--<boundary>"... is not valid JSON`. Drop the
        // `Content-Type` here (only for this multipart path — every other verb is
        // unchanged) so `.multipart()` can set the correct one.
        let mut headers = self.headers()?;
        headers.remove(CONTENT_TYPE);
        let response = self
            .client
            .post(url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        let value = if text.is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text).unwrap_or(Value::String(text))
        };
        if !status.is_success() {
            return Err(Error::Status {
                status: status.as_u16(),
                body: value,
            });
        }
        unwrap_envelope(value)
    }

    /// Send a request whose successful response is binary rather than JSON.
    pub async fn send_bytes(&self, method: Method, path: &str) -> Result<Vec<u8>, Error> {
        reject_unexposed_route(&method, path)?;
        let response = self
            .client
            .request(method, self.url(path, &[])?)
            .headers(self.headers()?)
            .send()
            .await?;
        let status = response.status();
        let bytes = response.bytes().await?;
        if !status.is_success() {
            let body = serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        Ok(bytes.to_vec())
    }

    fn url(&self, path: &str, query: &[QueryParam]) -> Result<Url, Error> {
        let normalized = if path.starts_with('/') {
            path.to_owned()
        } else {
            format!("/{path}")
        };
        let mut url = Url::parse(&format!("{}{}", self.base_url, normalized))?;
        let pairs: Vec<(&str, String)> = query
            .iter()
            .filter_map(|(k, v)| v.as_ref().map(|v| (*k, v.clone())))
            .collect();
        if !pairs.is_empty() {
            url.query_pairs_mut().extend_pairs(pairs);
        }
        Ok(url)
    }

    fn headers(&self) -> Result<HeaderMap, Error> {
        // Host-supplied headers go in first so the SDK's own headers below
        // overwrite them on conflict — see `with_default_headers`.
        let mut headers = self.default_headers.clone();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert("x-sdk-client", HeaderValue::from_static("tinyhumans-rust"));
        if let Some(token) = &self.token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {token}"))?,
            );
        }
        if let Some(api_key) = &self.api_key {
            headers.insert("x-api-key", HeaderValue::from_str(api_key)?);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}

/// Unwrap the hosted-backend `{success, data}` envelope.
///
/// `success: false` becomes [`Error::Envelope`] rather than a successful
/// value: the backend pairs a failed operation with an unsuccessful envelope
/// that is not always accompanied by a non-2xx status, so returning it as data
/// would hand the caller `{success: false, error: "..."}` where a result is
/// expected.
///
/// A successful envelope with no `data` key yields the remaining fields with
/// `success` removed, so envelopes that inline their payload (`{success, jwt}`)
/// do not leak the flag into the caller's value.
fn unwrap_envelope(body: Value) -> Result<Value, Error> {
    let Value::Object(mut map) = body else {
        return Ok(body);
    };
    match map.get("success").and_then(Value::as_bool) {
        Some(true) => {
            if let Some(data) = map.remove("data") {
                return Ok(data);
            }
            map.remove("success");
            Ok(Value::Object(map))
        }
        Some(false) => Err(Error::Envelope {
            error: map
                .get("error")
                .or_else(|| map.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("request unsuccessful")
                .to_owned(),
            error_code: map
                .get("errorCode")
                .and_then(Value::as_str)
                .map(str::to_owned),
            details: map.get("details").cloned().unwrap_or(Value::Null),
        }),
        None => Ok(Value::Object(map)),
    }
}

fn reject_unexposed_route(method: &Method, path: &str) -> Result<(), Error> {
    let blocked =
        route_matches(UNEXPOSED_ROUTES, method, path) || is_structurally_unexposed(method, path);
    if blocked {
        Err(Error::RouteNotExposed(
            method.as_str().to_owned(),
            path.to_owned(),
        ))
    } else {
        Ok(())
    }
}

fn route_matches(routes: &[(&str, &str)], method: &Method, path: &str) -> bool {
    let request_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();
    routes.iter().any(|(route_method, template)| {
        if *route_method != method.as_str() {
            return false;
        }
        let template_segments = template.trim_matches('/').split('/').collect::<Vec<_>>();
        template_segments.len() == request_segments.len()
            && template_segments
                .iter()
                .zip(&request_segments)
                .all(|(expected, actual)| {
                    (expected.starts_with('{') && expected.ends_with('}')) || expected == actual
                })
    })
}

fn is_structurally_unexposed(method: &Method, path: &str) -> bool {
    let segments = path.trim_matches('/').split('/').collect::<Vec<_>>();
    if segments.contains(&"admin") {
        return true;
    }

    // The deployed Swagger document omits webhook receivers entirely. Treat an
    // undocumented webhook route as a receiver; generated bearer-authenticated
    // routes such as `/webhooks/core*` remain available.
    segments.contains(&"webhooks") && !route_matches(PUBLIC_ROUTES, method, path)
}

#[cfg(test)]
mod exclusion_tests {
    use super::*;

    #[test]
    fn every_admin_and_webhook_route_is_rejected_by_the_raw_transport_gate() {
        assert_eq!(UNEXPOSED_ROUTES.len(), 44);
        for (method, template) in UNEXPOSED_ROUTES {
            let concrete_path = template
                .split('/')
                .map(|segment| {
                    if segment.starts_with('{') && segment.ends_with('}') {
                        "example"
                    } else {
                        segment
                    }
                })
                .collect::<Vec<_>>()
                .join("/");
            let method = Method::from_bytes(method.as_bytes()).unwrap();
            assert!(
                matches!(
                    reject_unexposed_route(&method, &concrete_path),
                    Err(Error::RouteNotExposed(_, _))
                ),
                "{} {template} was not blocked",
                method.as_str()
            );
        }
    }

    /// Team-scoped operations gated by the team-admin *role* are not platform
    /// administration and must stay reachable. Their OpenAPI summaries read
    /// "(admin only)", which the generator's admin heuristic would otherwise
    /// exclude — that broke OpenHuman's `team_remove_member`.
    #[test]
    fn team_role_gated_operations_are_not_treated_as_platform_admin() {
        for (method, path) in [
            ("PUT", "/teams/{teamId}"),
            ("DELETE", "/teams/{teamId}/members/{userId}"),
            ("PUT", "/teams/{teamId}/members/{userId}/role"),
        ] {
            assert!(
                !UNEXPOSED_ROUTES.contains(&(method, path)),
                "{method} {path} is team-role gated, not platform-admin"
            );
        }
    }

    /// Genuine platform-administration operations stay blocked, including the
    /// ones outside an `/admin` path segment that only their summary marks.
    #[test]
    fn platform_admin_operations_remain_blocked() {
        for (method, path) in [
            ("POST", "/coupons/admin"),
            ("GET", "/coupons/admin"),
            ("PATCH", "/feedback/{id}/status"),
            ("POST", "/invite/campaign"),
            ("DELETE", "/invite/campaign/{codeId}"),
            ("POST", "/agent-integrations/composio/toolkits/refresh"),
        ] {
            assert!(
                UNEXPOSED_ROUTES.contains(&(method, path)),
                "{method} {path} is platform-admin and must stay blocked"
            );
        }
    }

    /// The blocked set covers webhook *receivers* only. `/webhooks/core*` is
    /// user-owned tunnel CRUD — bearer-authenticated, user-facing, and driven
    /// by OpenHuman — so it must stay reachable even though it shares the
    /// `/webhooks` prefix with the receivers around it.
    #[test]
    fn webhook_tunnel_crud_is_not_in_the_blocked_set() {
        for (method, path) in [
            ("GET", "/webhooks/core"),
            ("POST", "/webhooks/core"),
            ("GET", "/webhooks/core/{id}"),
            ("PATCH", "/webhooks/core/{id}"),
            ("DELETE", "/webhooks/core/{id}"),
            ("GET", "/webhooks/core/bandwidth"),
        ] {
            assert!(
                !UNEXPOSED_ROUTES.contains(&(method, path)),
                "{method} {path} is user-facing and must not be blocked"
            );
        }
    }

    #[test]
    fn future_structurally_private_routes_are_rejected_without_regeneration() {
        for (method, path) in [
            (Method::POST, "/admin/future-operation"),
            (Method::POST, "/webhooks/future-provider"),
        ] {
            assert!(
                matches!(
                    reject_unexposed_route(&method, path),
                    Err(Error::RouteNotExposed(_, _))
                ),
                "{} {path} was not blocked",
                method.as_str()
            );
        }
    }

    #[test]
    fn generated_public_webhook_routes_pass_the_structural_guard() {
        for (method, path) in [
            (Method::GET, "/webhooks/core"),
            (Method::GET, "/webhooks/core/example"),
            (Method::GET, "/webhooks/core/bandwidth"),
        ] {
            assert!(
                reject_unexposed_route(&method, path).is_ok(),
                "{} {path} was blocked",
                method.as_str()
            );
        }
    }
}
