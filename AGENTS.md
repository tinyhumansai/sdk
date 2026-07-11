# SDK Development Guidelines

## What This Repo Is

This repository is the standalone TinyHumans SDK surface. It provides:

- Documentation for integrating with the TinyHumans backend.
- A shared backend namespace manifest in `api/tinyhumans.backend.json`.
- A TypeScript SDK and `tinyhumans` CLI in `sdk/typescript/`.
- Python and Rust SDKs in `sdk/python/` and `sdk/rust/`.
- Cross-language examples in `examples/`.

The backend contract source of truth is the deployed Swagger/OpenAPI document:

```text
https://api.tinyhumans.ai/swagger.json
```

Use the local backend source only as secondary implementation context. The SDK
surface should follow the deployed spec first.

## Backend Model

TinyHumans is an OpenAI-compatible inference proxy plus a team-billing and
agent-integration platform. The SDK should expose these API families cleanly:

- OpenAI-compatible inference under `/openai/*`.
- Auth and account state under `/auth/*`.
- User API key management under `/api-keys/*`.
- Team-scoped billing, credits, Stripe, and Coinbase routes under `/teams/*`
  and `/payments/*`.
- Agent integrations under `/agent-integrations/*` for Composio, Parallel,
  media generation, financial APIs, maps, Apify, Tenor, Twilio, and crypto.
- Telegram/Discord channel integration routes under `/channels/*` and
  provider webhook routes under `/webhooks/*`.
- Feedback, invites, referrals, rewards, announcements, mascots, investors,
  and admin analytics/content-management routes.

Successful JSON responses usually use:

```json
{ "success": true, "data": {} }
```

Errors usually use:

```json
{ "success": false, "error": "...", "errorCode": "...", "details": {} }
```

SDKs should unwrap successful envelopes by default but always allow callers to
request the raw response body.

## Project Layout

| Path | What it is |
| --- | --- |
| `api/tinyhumans.backend.json` | SDK-friendly summary of the deployed Swagger spec |
| `docs/` | Integration docs, auth, API surface, and CLI notes |
| `examples/` | Cross-language usage snippets |
| `sdk/typescript/` | npm package `@tinyhumansai/sdk` and CLI |
| `sdk/python/` | PyPI package `tinyhumans` |
| `sdk/rust/` | crates.io package `tinyhumans-sdk` |

## Build and Verification

Run the narrowest relevant checks for the package you changed:

```bash
pnpm --filter @tinyhumansai/sdk build
pnpm --filter @tinyhumansai/sdk lint
python -m py_compile sdk/python/src/tinyhumans/*.py
cargo check --manifest-path sdk/rust/Cargo.toml
```

For docs or manifest changes, also validate the deployed spec is reachable:

```bash
curl -fsSL https://api.tinyhumans.ai/swagger.json >/dev/null
```

When a package gains tests, add the package-native command here and keep it
passing before shipping changes.

## Code Conventions

- Keep SDKs dependency-light unless a dependency removes real protocol or
  runtime complexity.
- Keep public APIs consistent across TypeScript, Python, and Rust.
- Expose named namespace clients plus a raw escape hatch for newly deployed
  backend routes.
- Prefer explicit types and structured JSON values over ad hoc string parsing.
- Preserve the backend envelope semantics in all languages.
- Keep comments sparse and useful.
- Do not add direct database access or backend-only implementation details to
  SDK packages.

## Auth and Security

Supported credentials:

- `Authorization: Bearer <token>` for user/agent JWT flows.
- `x-api-key: <key>` for API-key routes.
- `x-admin-service-token: <token>` for trusted server-side admin write routes.

Never commit secrets, local tokens, generated credential files, or test keys.
Examples should read credentials from environment variables.

Admin service-token helpers must be documented as server-only. Do not present
them as browser-safe.

## OpenAPI Discipline

When adding or changing SDK methods:

1. Check `https://api.tinyhumans.ai/swagger.json` first.
2. Update `api/tinyhumans.backend.json` if a namespace or route family changed.
3. Add matching methods or namespace helpers in all language packages when
   feasible.
4. Keep docs and examples aligned with the deployed path names.

Do not invent stable SDK method names from local source alone if the deployed
Swagger operation has a different path or tag.

## Git and PR Expectations

- Keep changes small and coherent.
- Use conventional commit prefixes when committing: `feat:`, `fix:`, `docs:`,
  `test:`, `refactor:`, or `chore:`.
- Do not mix unrelated SDK language changes unless the API contract change
  requires cross-language parity.
