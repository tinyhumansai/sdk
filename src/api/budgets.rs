use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{enc, Error, HttpClient};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeatRequest {
    pub plan: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agent_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSeatRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

pub struct BudgetsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> BudgetsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }
    pub async fn get(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/budgets", &[], None, true)
            .await
    }
    pub async fn create_seat(&self, request: &CreateSeatRequest) -> Result<Value, Error> {
        let body = serde_json::to_value(request).expect("seat request is serializable");
        self.http
            .send(Method::POST, "/budgets/seats", &[], Some(&body), true)
            .await
    }
    pub async fn update_seat(
        &self,
        seat_id: &str,
        request: &UpdateSeatRequest,
    ) -> Result<Value, Error> {
        let path = format!("/budgets/seats/{}", enc(seat_id));
        let body = serde_json::to_value(request).expect("seat request is serializable");
        self.http
            .send(Method::PATCH, &path, &[], Some(&body), true)
            .await
    }
    pub async fn delete_seat(&self, seat_id: &str) -> Result<Value, Error> {
        let path = format!("/budgets/seats/{}", enc(seat_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }
}
