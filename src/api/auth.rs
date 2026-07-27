//! Auth and account state: magic-link login, OAuth, current user, and tokens.

use reqwest::Method;
use serde_json::Value;

use super::types::{EmailLinkRequest, IntegrationTokenRequest, LoginTokenRequest};
use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/auth/*` routes.
pub struct AuthApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AuthApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Create a short-lived token to link a Telegram or Discord account.
    pub async fn create_channel_link_token(&self, channel: &str) -> Result<Value, Error> {
        let path = format!("/auth/channels/{}/link-token", enc(channel));
        self.http.send(Method::POST, &path, &[], None, true).await
    }

    /// Send a magic link for email login.
    pub async fn send_email_link(&self, request: &EmailLinkRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("email link request is serializable");
        self.http
            .send(
                Method::POST,
                "/auth/email/send-link",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// Verify a magic link and complete email login (302 redirect).
    pub async fn verify_email(&self, token: &str) -> Result<Value, Error> {
        let query: [QueryParam; 1] = [("token", Some(token.to_string()))];
        self.http
            .send(Method::GET, "/auth/email/verify", &query, None, true)
            .await
    }

    /// Consume a one-time login token, exchanging it for a session.
    pub async fn consume_login_token(&self, request: &LoginTokenRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("login token request is serializable");
        self.http
            .send(
                Method::POST,
                "/auth/login-token/consume",
                &[],
                Some(&body),
                true,
            )
            .await
    }

    /// Get the currently authenticated user.
    pub async fn me(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/auth/me", &[], None, true)
            .await
    }

    /// List the user's connected third-party integrations.
    pub async fn list_integrations(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/auth/integrations", &[], None, true)
            .await
    }

    /// Disconnect a third-party integration.
    pub async fn delete_integration(&self, integration_id: &str) -> Result<Value, Error> {
        let path = format!("/auth/integrations/{}", enc(integration_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Issue an access token for a connected integration.
    pub async fn create_integration_token(
        &self,
        integration_id: &str,
        request: &IntegrationTokenRequest,
    ) -> Result<Value, Error> {
        let body =
            serde_json::to_value(request).expect("integration token request is serializable");
        let path = format!("/auth/integrations/{}/tokens", enc(integration_id));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// OAuth provider callback endpoint (returns a redirect).
    pub async fn oauth_callback(&self, provider: &str) -> Result<Value, Error> {
        let path = format!("/auth/{}/callback", enc(provider));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Start an OAuth connect flow for a provider.
    pub async fn oauth_connect(
        &self,
        provider: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/auth/{}/connect", enc(provider));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Start an OAuth login flow for a provider (returns a redirect).
    pub async fn oauth_login(&self, provider: &str, query: &[QueryParam]) -> Result<Value, Error> {
        let path = format!("/auth/{}/login", enc(provider));
        self.http.send(Method::GET, &path, query, None, true).await
    }
}
