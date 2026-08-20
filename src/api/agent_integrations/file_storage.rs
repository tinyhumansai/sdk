//! Agent file storage: upload, download, visibility, and share links.

use super::AgentIntegrationsApi;
use crate::{enc, Error, QueryParam};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileMetadata {
    pub file_id: String,
    pub filename: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub visibility: String,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub public_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct StorageUsage {
    #[serde(default, rename = "usedBytes")]
    pub used_bytes: u64,
    #[serde(default, rename = "limitBytes")]
    pub limit_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ListFilesResponse {
    #[serde(default)]
    pub files: Vec<FileMetadata>,
    #[serde(default)]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub usage: StorageUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    #[serde(flatten)]
    pub file: FileMetadata,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FileLinkResponse {
    pub url: String,
    #[serde(default, rename = "expiresAt")]
    pub expires_at: Option<String>,
    #[serde(default, rename = "costUsd")]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteFileResponse {
    #[serde(default)]
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateFileVisibilityRequest {
    pub visibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateFileLinkRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_seconds: Option<u32>,
}

impl AgentIntegrationsApi<'_> {
    pub async fn list_files(&self, query: &[QueryParam]) -> Result<ListFilesResponse, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/file-storage/files",
                query,
                None,
                true,
            )
            .await
    }
    pub async fn get_file(&self, file_id: &str) -> Result<FileMetadata, Error> {
        self.http
            .send_typed(
                Method::GET,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn download_file(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.http
            .send_bytes(
                Method::GET,
                &format!(
                    "/agent-integrations/file-storage/files/{}/download",
                    enc(file_id)
                ),
            )
            .await
    }
    pub async fn public_file(&self, file_id: &str) -> Result<Vec<u8>, Error> {
        self.http
            .send_bytes(
                Method::GET,
                &format!("/agent-integrations/file-storage/public/{}", enc(file_id)),
            )
            .await
    }
    pub async fn file_storage_usage(&self) -> Result<StorageUsage, Error> {
        self.http
            .send_typed(
                Method::GET,
                "/agent-integrations/file-storage/usage",
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn upload_file(
        &self,
        file_name: &str,
        bytes: Vec<u8>,
        visibility: Option<&str>,
        ttl_days: Option<u32>,
    ) -> Result<UploadFileResponse, Error> {
        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(bytes).file_name(file_name.to_owned()),
        );
        if let Some(value) = visibility {
            form = form.text("visibility", value.to_owned());
        }
        if let Some(value) = ttl_days {
            form = form.text("ttlDays", value.to_string());
        }
        let value = self
            .http
            .post_multipart("/agent-integrations/file-storage/files", form)
            .await?;
        Ok(serde_json::from_value(value)?)
    }
    pub async fn update_file_visibility(
        &self,
        file_id: &str,
        visibility: &str,
    ) -> Result<FileMetadata, Error> {
        let body = serde_json::to_value(UpdateFileVisibilityRequest {
            visibility: visibility.to_owned(),
        })?;
        self.send(
            Method::PATCH,
            &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
            &[],
            Some(&body),
            true,
        )
        .await
    }
    pub async fn delete_file(&self, file_id: &str) -> Result<DeleteFileResponse, Error> {
        self.http
            .send_typed(
                Method::DELETE,
                &format!("/agent-integrations/file-storage/files/{}", enc(file_id)),
                &[],
                None,
                true,
            )
            .await
    }
    pub async fn create_file_link(
        &self,
        file_id: &str,
        expires_in_seconds: Option<u32>,
    ) -> Result<FileLinkResponse, Error> {
        self.post(
            &format!(
                "/agent-integrations/file-storage/files/{}/link",
                enc(file_id)
            ),
            &CreateFileLinkRequest { expires_in_seconds },
        )
        .await
    }
}
