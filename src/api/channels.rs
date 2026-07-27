//! Telegram and Discord channel integration: messages, reactions, threads, typing.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/channels/*` routes.
pub struct ChannelsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> ChannelsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Send a rich message to a user's linked channel.
    pub async fn send_message(&self, channel: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/channels/{}/messages", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// Delete a message from a user's linked channel.
    pub async fn delete_message(&self, channel: &str, message_id: &str) -> Result<Value, Error> {
        let path = format!("/channels/{}/messages/{}", enc(channel), enc(message_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// React to a message on a user's linked channel.
    pub async fn add_reaction(&self, channel: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/channels/{}/reactions", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// Create a new conversation thread.
    pub async fn create_thread(&self, channel: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/channels/{}/threads", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// List conversation threads.
    pub async fn list_threads(&self, channel: &str, query: &[QueryParam]) -> Result<Value, Error> {
        let path = format!("/channels/{}/threads", enc(channel));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Update a thread's status (close or reopen).
    pub async fn update_thread(
        &self,
        channel: &str,
        thread_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!("/channels/{}/threads/{}", enc(channel), enc(thread_id));
        self.http
            .send(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// Broadcast a typing indicator on a user's linked channel.
    pub async fn send_typing(&self, channel: &str) -> Result<Value, Error> {
        let path = format!("/channels/{}/typing", enc(channel));
        self.http.send(Method::POST, &path, &[], None, true).await
    }
}
