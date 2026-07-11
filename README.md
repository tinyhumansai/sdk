# TinyHumans SDK

SDKs and tools for integrating with the TinyHumans backend.

This repository mirrors the SDK layout used by `../tiny.place/frontend/sdk`: a
top-level documentation and examples surface, a CLI, and language bindings for
TypeScript, Python, and Rust.

## Contents

| Path | What it is |
| --- | --- |
| [`api/tinyhumans.backend.json`](api/tinyhumans.backend.json) | Shared backend namespace manifest derived from the deployed Swagger spec |
| [`docs/`](docs/README.md) | Integration docs, API surface, auth, and CLI usage |
| [`examples/`](examples/README.md) | Small cross-language examples |
| [`sdk/typescript/`](sdk/typescript/README.md) | TypeScript SDK and `tinyhumans` CLI |
| [`sdk/python/`](sdk/python/README.md) | Python SDK |
| [`sdk/rust/`](sdk/rust/README.md) | Rust SDK crate |

## Quick Start

```bash
pnpm --filter @tinyhumansai/sdk build
pnpm --filter @tinyhumansai/sdk exec tinyhumans health --base-url https://api.tinyhumans.ai
```

```ts
import { TinyHumansClient } from "@tinyhumansai/sdk";

const client = new TinyHumansClient({
  baseUrl: "https://api.tinyhumans.ai",
  token: process.env.TINYHUMANS_TOKEN,
});

const models = await client.inference.get("/v1/models");
```

```python
import os
from tinyhumans import TinyHumansClient

client = TinyHumansClient(
    base_url="https://api.tinyhumans.ai",
    token=os.environ.get("TINYHUMANS_TOKEN"),
)
print(client.inference.get("/v1/models"))
```

```rust
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new("https://api.tinyhumans.ai")
    .with_token(std::env::var("TINYHUMANS_TOKEN").ok());
let models = client.inference().get("/v1/models").await?;
# Ok(())
# }
```

## Backend Surface

The current manifest is grounded in the deployed backend OpenAPI document:
<https://api.tinyhumans.ai/swagger.json>. The live spec currently advertises
TinyHumans API `1.0.0`, production and staging servers, and 172 paths. It
captures the stable public domains:

- Health and documentation: `/`, `/swagger.json`
- Auth, API keys, and account state: `/auth`, `/api-keys`
- Inference and OpenAI-compatible routes: `/openai`
- Agent integrations: `/agent-integrations`
- Payments, credits, coupons, referrals, invites, and rewards
- Feedback, teams, channels, mascots, rewards, announcements, and admin routes
- Webhook ingress routes for provider callbacks

Each SDK exposes a `raw` request helper and named namespace clients so new
backend endpoints can be used before a fully typed convenience method is added.
