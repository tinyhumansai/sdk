# TinyHumans Rust SDK

Rust client for the public TinyHumans backend API.

The crate is grounded in the deployed OpenAPI document at
<https://api.tinyhumans.ai/swagger.json> plus five public operations exercised
by OpenHuman but not yet documented there. It exposes 197 public operations
across 21 namespaces. All 35 administrative operations are intentionally
excluded, including legacy routes outside `/admin` whose contract marks them
as admin-only.

## Layout

| Path | Purpose |
| --- | --- |
| `Cargo.toml` | Crate manifest for `tinyhumans-sdk` |
| `src/` | HTTP client, typed namespace clients, and generated public route registry |
| `tests/` | Transport, serialization, route, and OpenAPI parity tests |
| `examples/` | Rust usage and live-contract examples |
| `api/tinyhumans.backend.json` | Generated public namespace manifest |
| `scripts/sync-openapi.mjs` | Deterministic OpenAPI filter and Rust route generator |

## Quick start

```rust
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new("https://api.tinyhumans.ai")
    .with_token(std::env::var("TINYHUMANS_TOKEN").ok());
let models = client.inference().list_models().await?;
# Ok(())
# }
```

## Streaming

The SDK includes a reconnecting Medulla SSE session stream and an authenticated
Socket.IO client for the full live backend event catalog. The socket client has
generic JSON/binary/ack primitives plus typed Medulla harness and workflow
helpers. See [docs/streaming.md](docs/streaming.md).

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo package

node scripts/sync-openapi.mjs
node scripts/sync-openapi.mjs --check
```

The crate retains a raw HTTP escape hatch for newly deployed public routes,
but no admin credential helper or typed admin route is provided.
