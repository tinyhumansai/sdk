use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as ReqwestClient, Method};
use serde::Serialize;
use serde_json::Value;
use url::Url;

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

    pub fn with_admin_service_token(mut self, admin_service_token: Option<String>) -> Self {
        self.http.admin_service_token = admin_service_token;
        self
    }

    pub async fn health(&self) -> Result<Value, Error> {
        self.http.get("/").await
    }

    pub async fn swagger(&self) -> Result<Value, Error> {
        self.http.get_unwrapped("/swagger.json", false).await
    }

    pub fn raw(&self) -> &HttpClient {
        &self.http
    }

    pub fn api_keys(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/api-keys")
    }

    pub fn auth(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/auth")
    }

    pub fn inference(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/openai")
    }

    pub fn agent_integrations(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/agent-integrations")
    }

    pub fn payments(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/payments")
    }

    pub fn feedback(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/feedback")
    }

    pub fn teams(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/teams")
    }

    pub fn channels(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/channels")
    }

    pub fn mascots(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/mascots")
    }

    pub fn admin(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/admin")
    }

    pub fn announcements(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/announcements")
    }

    pub fn coupons(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/coupons")
    }

    pub fn invite(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/invite")
    }

    pub fn investors(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/investors")
    }

    pub fn referral(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/referral")
    }

    pub fn rewards(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/rewards")
    }

    pub fn webhooks(&self) -> NamespaceClient<'_> {
        NamespaceClient::new(&self.http, "/webhooks")
    }
}

#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
    token: Option<String>,
    api_key: Option<String>,
    admin_service_token: Option<String>,
    client: ReqwestClient,
}

impl HttpClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            token: None,
            api_key: None,
            admin_service_token: None,
            client: ReqwestClient::new(),
        }
    }

    pub async fn get(&self, path: &str) -> Result<Value, Error> {
        self.request(Method::GET, path, Option::<&Value>::None).await
    }

    pub async fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.request(Method::POST, path, Some(body)).await
    }

    pub async fn put<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.request(Method::PUT, path, Some(body)).await
    }

    pub async fn patch<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<Value, Error> {
        self.request(Method::DELETE, path, Option::<&Value>::None).await
    }

    pub async fn get_unwrapped(&self, path: &str, unwrap_envelope: bool) -> Result<Value, Error> {
        self.request_unwrapped(Method::GET, path, Option::<&Value>::None, unwrap_envelope)
            .await
    }

    async fn request<T: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<Value, Error> {
        self.request_unwrapped(method, path, body, true).await
    }

    async fn request_unwrapped<T: Serialize + ?Sized>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
        unwrap_envelope: bool,
    ) -> Result<Value, Error> {
        let url = self.url(path)?;
        let mut request = self.client.request(method, url).headers(self.headers()?);
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        let body = if text.is_empty() {
            Value::Null
        } else {
            serde_json::from_str(&text).unwrap_or(Value::String(text))
        };
        if !status.is_success() {
            return Err(Error::Status {
                status: status.as_u16(),
                body,
            });
        }
        Ok(if unwrap_envelope {
            unwrap(body)
        } else {
            body
        })
    }

    fn url(&self, path: &str) -> Result<Url, Error> {
        let normalized = if path.starts_with('/') {
            path.to_owned()
        } else {
            format!("/{path}")
        };
        Ok(Url::parse(&format!("{}{}", self.base_url, normalized))?)
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
        if let Some(token) = &self.admin_service_token {
            headers.insert("x-admin-service-token", HeaderValue::from_str(token)?);
        }
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok(headers)
    }
}

pub struct NamespaceClient<'a> {
    http: &'a HttpClient,
    base_path: &'static str,
}

impl<'a> NamespaceClient<'a> {
    fn new(http: &'a HttpClient, base_path: &'static str) -> Self {
        Self { http, base_path }
    }

    pub async fn get(&self, path: &str) -> Result<Value, Error> {
        self.http.get(&self.path(path)).await
    }

    pub async fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.http.post(&self.path(path), body).await
    }

    pub async fn put<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.http.put(&self.path(path), body).await
    }

    pub async fn patch<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Value, Error> {
        self.http.patch(&self.path(path), body).await
    }

    pub async fn delete(&self, path: &str) -> Result<Value, Error> {
        self.http.delete(&self.path(path)).await
    }

    fn path(&self, path: &str) -> String {
        let suffix = if path == "/" {
            ""
        } else if path.starts_with('/') {
            path
        } else {
            return format!("{}/{}", self.base_path, path);
        };
        format!("{}{}", self.base_path, suffix)
    }
}

fn unwrap(body: Value) -> Value {
    match body {
        Value::Object(mut map) if map.get("success") == Some(&Value::Bool(true)) => {
            map.remove("data").unwrap_or(Value::Object(map))
        }
        other => other,
    }
}
