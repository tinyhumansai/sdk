# TypeScript SDK

```bash
npm install @tinyhumansai/sdk
```

```ts
import { TinyHumansClient } from "@tinyhumansai/sdk";

const client = new TinyHumansClient({
  baseUrl: "https://api.tinyhumans.ai",
  token: process.env.TINYHUMANS_TOKEN,
});

const me = await client.auth.get("/me");
const models = await client.inference.get("/v1/models");
```

The package also ships a CLI:

```bash
pnpm exec tinyhumans health --base-url https://api.tinyhumans.ai
pnpm exec tinyhumans get /auth/me --token "$TINYHUMANS_TOKEN"
```

Namespace clients are intentionally thin wrappers over the deployed Swagger
contract at <https://api.tinyhumans.ai/swagger.json>. Use `client.raw.request`
for a newly deployed route before a convenience helper exists.
