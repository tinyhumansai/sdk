# SDK Development Guidelines

## What This Repo Is

This repository is the standalone TinyHumans Rust SDK. It provides:

- Documentation for integrating with the TinyHumans backend.
- A shared backend namespace manifest in `api/tinyhumans.backend.json`.
- A root Cargo crate published as `tinyhumans-sdk`.
- Rust examples in `examples/`.

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
- Telegram/Discord channel integration routes under `/channels/*`.
- Feedback, invites, referrals, rewards, announcements, mascots, Medulla,
  OpenCompany, and orchestration routes.

Administrative and webhook routes are intentionally excluded, including legacy
operations outside `/admin` whose deployed OpenAPI summary marks them as
admin-only. Never add typed methods for these routes or permit them through the
raw transport.

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
| `docs/` | Integration docs, auth, API surface, and release notes |
| `examples/` | Rust usage and live-contract examples |
| `src/` | Rust transport and namespace clients |
| `tests/` | Rust integration and contract tests |

## Build and Verification

Run the narrowest relevant checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo package
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
- Expose named namespace clients plus a raw escape hatch for newly deployed
  public backend routes.
- Prefer explicit types and structured JSON values over ad hoc string parsing.
- Preserve the backend envelope semantics in the Rust transport.
- Keep comments sparse and useful.
- Do not add direct database access or backend-only implementation details to
  SDK packages.

## Auth and Security

Supported credentials:

- `Authorization: Bearer <token>` for user/agent JWT flows.
- `x-api-key: <key>` for API-key routes.

Never commit secrets, local tokens, generated credential files, or test keys.
Examples should read credentials from environment variables.

Do not add admin service-token helpers or administrative operations to the
public SDK packages.

## OpenAPI Discipline

When adding or changing SDK methods:

1. Check `https://api.tinyhumans.ai/swagger.json` first.
2. Update `api/tinyhumans.backend.json` if a namespace or route family changed.
3. Add matching Rust methods, request/response types, and route tests.
4. Keep docs and examples aligned with the deployed path names.

Do not invent stable SDK method names from local source alone if the deployed
Swagger operation has a different path or tag.

Never add an administrative route or admin credential helper to any SDK,
including legacy paths outside `/admin` whose summary, description, or 403
response marks them as admin-only. `node scripts/sync-openapi.mjs` must retain that filter
and regenerate both the manifest and Rust public-route registry.

`sync-openapi.mjs` defaults to fetching the deployed spec. When syncing a backend
branch that adds or changes routes, dump that checkout's spec first and pass it
with `--input` (`cd ../backend && npm run swagger -- /tmp/spec.json`); a bare run
regenerates from production and reverts the branch's routes back out. Feed it the
RAW document, never the filtered one served at `/swagger.json` — this script does
its own admin/webhook exclusion and derives `UNEXPOSED_ROUTES` from what it sees,
so a pre-filtered input shrinks that denylist and unblocks the routes it exists to
block. `UNEXPOSED_ROUTES.len()` is pinned in `src/lib.rs` to catch exactly that.

## Git and PR Expectations

- Keep changes small and coherent.
- Use conventional commit prefixes when committing: `feat:`, `fix:`, `docs:`,
  `test:`, `refactor:`, or `chore:`.
- Keep Rust client, tests, generated route registry, and docs in the same
  contract change.
