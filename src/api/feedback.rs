//! Public feedback board: submit, browse, read, vote, and comment.

use reqwest::Method;
use serde_json::Value;

use super::types::{
    CreateFeedbackRequest, FeedbackCommentRequest, FeedbackVoteRequest, IngestFeedbackRequest,
};
use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/feedback/*` routes.
pub struct FeedbackApi<'a> {
    http: &'a HttpClient,
}

impl<'a> FeedbackApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Submit feedback or a bug report (LLM-moderated, rate-limited).
    pub async fn create_feedback(&self, request: &CreateFeedbackRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("feedback request is serializable");
        self.http
            .send(Method::POST, "/feedback", &[], Some(&body), true)
            .await
    }

    pub async fn ingest_feedback(&self, request: &IngestFeedbackRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("feedback ingest request is serializable");
        self.http
            .send(Method::POST, "/feedback/ingest", &[], Some(&body), true)
            .await
    }

    /// List feedback on the public board.
    pub async fn list_feedback(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/feedback", query, None, true)
            .await
    }

    /// Get a feedback item with its comments.
    pub async fn get_feedback(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/feedback/{}", enc(id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Comment on a feedback item.
    pub async fn comment_feedback(
        &self,
        id: &str,
        request: &FeedbackCommentRequest,
    ) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("feedback comment is serializable");
        let path = format!("/feedback/{}/comments", enc(id));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// Up/down-vote a feedback item (value 0 retracts).
    pub async fn vote_feedback(
        &self,
        id: &str,
        request: &FeedbackVoteRequest,
    ) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("feedback vote is serializable");
        let path = format!("/feedback/{}/vote", enc(id));
        self.http
            .send(Method::POST, &path, &[], Some(&body), true)
            .await
    }
}
