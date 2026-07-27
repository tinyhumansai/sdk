# TinyHumans SDK Docs

This SDK repository defines one backend integration contract and exposes it
through TypeScript, Python, Rust, and a CLI.

## Documents

- [API surface](api-surface.md): backend namespaces and authentication shape.
- [Authentication](auth.md): bearer tokens, API keys, and headers shared
  across SDKs.
- [CLI](cli.md): `tinyhumans` command usage.

## Design

The backend surface is broad and changes faster than hand-written SDK methods.
Every language binding therefore exposes:

- A shared request pipeline with JSON envelope unwrapping.
- Typed namespace clients such as `auth`, `inference`, `payments`, `feedback`,
  and `agentIntegrations`, with one method per public deployed operation (201
  across 21 namespaces).
- A raw request method for new backend endpoints before typed methods land.

Administrative operations are intentionally outside the SDK surface, including
legacy routes whose OpenAPI summary marks them as admin-only.

The source of truth for deployed backend behavior is
<https://api.tinyhumans.ai/swagger.json>. The local manifest at
[`../api/tinyhumans.backend.json`](../api/tinyhumans.backend.json) summarizes
that Swagger/OpenAPI contract for SDK authors.
