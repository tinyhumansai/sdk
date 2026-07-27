use std::env;

use tinyhumans_sdk::{Error, TinyHumansClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("TINYHUMANS_E2E_BASE_URL").unwrap_or_else(|_| "http://localhost:5000".to_owned());
    let client = TinyHumansClient::new(&base_url);

    let health = client.health().check().await?;
    if health.get("online").and_then(|value| value.as_bool()) != Some(true) {
        return Err(format!("unexpected health payload from {base_url}: {health}").into());
    }

    let swagger = client.swagger().await?;
    let path_count = swagger
        .get("paths")
        .and_then(|value| value.as_object())
        .map_or(0, |paths| paths.len());
    if path_count == 0 {
        return Err("Swagger document contains no paths".into());
    }

    match client.api_keys().list().await {
        Err(Error::Status { status, .. }) if status == 401 || status == 403 => {}
        Err(error) => {
            return Err(format!("unexpected unauthenticated /api-keys error: {error}").into())
        }
        Ok(value) => {
            return Err(
                format!("/api-keys unexpectedly allowed unauthenticated access: {value}").into(),
            )
        }
    }

    println!("Rust SDK live contract passed against {base_url} ({path_count} Swagger paths)");
    Ok(())
}
