# Rust SDK

```toml
tinyhumans-sdk = "0.1"
```

```rust
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new("https://api.tinyhumans.ai")
    .with_token(std::env::var("TINYHUMANS_TOKEN").ok());

let me = client.auth().get("/me").await?;
let models = client.inference().get("/v1/models").await?;
# Ok(())
# }
```

The Rust crate exposes thin namespace clients and a raw request pipeline over
the deployed Swagger contract at <https://api.tinyhumans.ai/swagger.json>.
Its typed surface intentionally follows the non-admin contract, including API
keys, budgets, file storage, Medulla, OpenCompany, and orchestration. Admin
operations and admin credentials are not exposed.
