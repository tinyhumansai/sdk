# API Surface

The SDK surface is grounded in the deployed Swagger/OpenAPI contract at
<https://api.tinyhumans.ai/swagger.json>. At the time this scaffold was written,
the spec reports TinyHumans API `1.0.0` with 172 paths.

| Namespace | Base path | Auth | Examples |
| --- | --- | --- | --- |
| `health` | `/`, `/swagger.json` | none | liveness, swagger |
| `apiKeys` | `/api-keys` | bearer | create, list, revoke, usage |
| `auth` | `/auth` | bearer | email login, OAuth, `GET /me`, integration tokens |
| `inference` | `/openai` | bearer | `GET /v1/models`, chat completions, responses, transcription |
| `agentIntegrations` | `/agent-integrations` | bearer | Composio, Parallel, media generation, maps, Apify, Twilio, crypto |
| `payments` | `/payments` | bearer | Stripe, Coinbase, credits, transactions, plans |
| `feedback` | `/feedback` | bearer | create, list, detail, vote, comments, status |
| `teams` | `/teams` | bearer | team list/detail/update, usage, invites, join |
| `channels` | `/channels` | bearer | messages, reactions, typing, threads |
| `mascots` | `/mascots` | mixed | catalog, render streams, meetings, Rive assets |
| `admin` | `/admin` | bearer or service token | analytics, users, credits, content, audit logs |
| `announcements` | `/announcements` | bearer | latest active announcement |
| `coupons` | `/coupons` | bearer | redemption and coupon history |
| `invite` | `/invite` | mixed | status, redemption, campaign invites |
| `investors` | `/investors` | none | deck lookup and event tracking |
| `referral` | `/referral` | bearer | referral stats and claim |
| `rewards` | `/rewards` | bearer | reward snapshot and Discord unlink |
| `webhooks` | `/webhooks` | mixed | provider callbacks and webhook tunnels |

Most JSON responses use the hosted-backend envelope:

```json
{
  "success": true,
  "data": {}
}
```

SDK request helpers unwrap this envelope by default. Raw helpers can return the
full response body when callers need status metadata or non-standard payloads.
