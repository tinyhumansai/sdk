//! Telegram and Discord channel integration: messages, reactions, threads, typing.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use super::types::DynamicResponse;
use crate::{enc, Error, HttpClient, QueryParam};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChannelIdentifier {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParseMode {
    Markdown,
    Plain,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MessageButton {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelIdentifier>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<MessageButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<ChannelIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddReactionRequest {
    pub message_id: ChannelIdentifier,
    pub emoji: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChannelIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateThreadRequest {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThreadAction {
    Close,
    Reopen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateThreadRequest {
    pub action: ThreadAction,
}

/// Typed client for the `/channels/*` routes.
pub struct ChannelsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> ChannelsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Send a rich message to a user's linked channel.
    pub async fn send_message(
        &self,
        channel: &str,
        request: &SendMessageRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("channel message request is serializable");
        let path = format!("/channels/{}/messages", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }

    /// Delete a message from a user's linked channel.
    pub async fn delete_message(
        &self,
        channel: &str,
        message_id: &str,
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/channels/{}/messages/{}", enc(channel), enc(message_id));
        self.http
            .send(Method::DELETE, &path, &[], None, true)
            .await
            .map(Into::into)
    }

    /// React to a message on a user's linked channel.
    pub async fn add_reaction(
        &self,
        channel: &str,
        request: &AddReactionRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("reaction request is serializable");
        let path = format!("/channels/{}/reactions", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }

    /// Create a new conversation thread.
    pub async fn create_thread(
        &self,
        channel: &str,
        request: &CreateThreadRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("thread request is serializable");
        let path = format!("/channels/{}/threads", enc(channel));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }

    /// List conversation threads.
    pub async fn list_threads(
        &self,
        channel: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/channels/{}/threads", enc(channel));
        self.http
            .send(Method::GET, &path, query, None, true)
            .await
            .map(Into::into)
    }

    /// Update a thread's status (close or reopen).
    pub async fn update_thread(
        &self,
        channel: &str,
        thread_id: &str,
        request: &UpdateThreadRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("thread update request is serializable");
        let path = format!("/channels/{}/threads/{}", enc(channel), enc(thread_id));
        self.http
            .send(Method::PATCH, &path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }

    /// Broadcast a typing indicator on a user's linked channel.
    pub async fn send_typing(&self, channel: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/channels/{}/typing", enc(channel));
        self.http
            .send(Method::POST, &path, &[], None, true)
            .await
            .map(Into::into)
    }
}
