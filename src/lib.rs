//! Rust SDK for the TinyHumans backend.
//!
//! [`TinyHumansClient`] exposes one typed namespace accessor per backend area,
//! each with one method per deployed operation. [`TinyHumansClient::raw`] is the
//! escape hatch for routes not yet surfaced as typed methods.

use generated_public_routes::UNEXPOSED_ROUTES;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as ReqwestClient, Method};
use serde_json::Value;
use url::Url;

pub mod api;
pub mod generated_public_routes;

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
    #[error("route is intentionally not exposed by the SDK: {0} {1}")]
    RouteNotExposed(String, String),
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
}

#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
    token: Option<String>,
    api_key: Option<String>,
    client: ReqwestClient,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            token: None,
            api_key: None,
            client: ReqwestClient::new(),
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
        Ok(if unwrap {
            unwrap_envelope(value)
        } else {
            value
        })
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
        let response = self
            .client
            .post(url)
            .headers(self.headers()?)
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
        Ok(unwrap_envelope(value))
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
        let mut headers = HeaderMap::new();
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

fn unwrap_envelope(body: Value) -> Value {
    match body {
        Value::Object(mut map) if map.get("success") == Some(&Value::Bool(true)) => {
            map.remove("data").unwrap_or(Value::Object(map))
        }
        other => other,
    }
}

fn reject_unexposed_route(method: &Method, path: &str) -> Result<(), Error> {
    let request_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();
    let blocked = UNEXPOSED_ROUTES.iter().any(|(blocked_method, template)| {
        if *blocked_method != method.as_str() {
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
    });
    if blocked {
        Err(Error::RouteNotExposed(
            method.as_str().to_owned(),
            path.to_owned(),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod exclusion_tests {
    use super::*;

    #[test]
    fn every_admin_and_webhook_route_is_rejected_by_the_raw_transport_gate() {
        // Pinned so the list can only ever be reviewed upward. A regenerated
        // spec that stopped describing admin or webhook routes would otherwise
        // shrink this list and silently unblock them at the raw transport.
        assert_eq!(UNEXPOSED_ROUTES.len(), 58);
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
}
