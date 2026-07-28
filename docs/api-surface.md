# API Surface

The SDK surface is grounded in the deployed Swagger/OpenAPI contract at
<https://api.tinyhumans.ai/swagger.json>. The spec reports TinyHumans API
`1.0.0` with 205 paths and 235 operations. The Rust SDK exposes one typed
method per public operation — **196 operations across the 21 namespaces
below**.
The remaining 32 administrative and 12 webhook-receiver operations are
intentionally excluded, including legacy routes whose summaries explicitly say
they are admin-only.

"Administrative" means *platform* administration. It does not cover operations
gated by a role within a resource the caller belongs to: `PUT /teams/{teamId}`,
`DELETE /teams/{teamId}/members/{userId}`, and
`PUT /teams/{teamId}/members/{userId}/role` say "(admin only)" in the contract,
but that is the **team-admin role** — held by any user who creates a team — not
platform administrator rights. They are exposed on the `teams` namespace.

"Webhook" exclusion means webhook *receivers* — the endpoints providers call
into (Stripe, Telegram, Discord, GitHub, Composio, Coinbase, Sentry, Twilio,
and the tunnel ingress paths). They carry no `bearerAuth`, are authenticated by
provider signature, and an SDK caller invoking one would be forging provider
traffic. The user-owned webhook *tunnel* CRUD under `/webhooks/core` is
ordinary bearer-authenticated user-facing API and is exposed as the `webhooks`
namespace.

| Namespace | Base path | Auth | Examples |
| --- | --- | --- | --- |
| `health` | `/` | none | `check()` liveness |
| `auth` | `/auth` | bearer | email login, OAuth, `me()`, integration tokens |
| `inference` | `/openai` | bearer | `GET /v1/models`, chat completions, responses, transcription |
| `agentIntegrations` | `/agent-integrations` | bearer | Composio, Parallel, media generation, maps, Apify, Twilio, crypto |
| `apiKeys` | `/api-keys` | bearer | create, list, and revoke user API keys |
| `budgets` | `/budgets` | bearer | team budgets and seat allocations |
| `medulla` | `/medulla` | bearer | roster, routing, sessions, messages, tasks, and sources |
| `openCompany` | `/opencompany` | bearer | company instances, lifecycle, and custom domains |
| `orchestration` | `/orchestration` | bearer | runs, events, sessions, state, and world diffs |
| `payments` | `/payments` | bearer | Stripe, Coinbase, credits, transactions, plans |
| `feedback` | `/feedback` | bearer | create, ingest, list, detail, vote, and comments |
| `teams` | `/teams` | bearer | team list/detail, usage, invites, join, leave, and billing |
| `channels` | `/channels` | bearer | messages, reactions, typing, threads |
| `mascots` | `/mascots` | mixed | catalog, render streams, meetings, Rive assets |
| `announcements` | `/announcements` | bearer | latest active announcement |
| `coupons` | `/coupons` | bearer | redemption and coupon history |
| `invite` | `/invite` | mixed | invite status, redemption, and owned codes |
| `referral` | `/referral` | bearer | referral stats and claim |
| `rewards` | `/rewards` | bearer | reward snapshot and Discord unlink |
| `redirect` | `/r` | none | resolve short redirect codes |
| `webhooks` | `/webhooks` | bearer | webhook tunnel CRUD and bandwidth budget |

The checked-in namespace manifest and Rust `PUBLIC_ROUTES` registry are
generated deterministically:

```bash
node scripts/sync-openapi.mjs
node scripts/sync-openapi.mjs --check
```

Most JSON responses use the hosted-backend envelope:

```json
{
  "success": true,
  "data": {}
}
```

SDK request helpers unwrap this envelope by default. The raw helper can return the
full response body when callers need status metadata or non-standard payloads.
