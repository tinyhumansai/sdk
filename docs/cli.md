# CLI

The TypeScript package ships the `tinyhumans` command. It mirrors the typed SDK:
every namespace method is a `tinyhumans <namespace> <command>` subcommand
(193 commands across 18 namespaces), with a `raw` escape hatch for anything not
yet typed.

## Discovering commands

```bash
tinyhumans                       # top-level usage + namespace list
tinyhumans list                  # every command, grouped by namespace
tinyhumans auth                  # list the auth namespace's commands
tinyhumans auth verify-email --help   # show a command's signature
```

## Argument model

Each command maps to its typed method's parameters:

- **positional params** are passed in order (or as `--<name> VALUE`),
- the **request body** comes from `--body JSON` or piped stdin,
- **query params** come from repeatable `--query key=value`.

```bash
# GET /auth/me
tinyhumans auth me --token "$TINYHUMANS_TOKEN"

# GET /auth/email/verify?token=... (token is a positional param)
tinyhumans auth verify-email tok_123

# PUT /teams/:teamId with a JSON body
tinyhumans teams update-team team_9 --body '{"name":"New name"}' --token "$TINYHUMANS_TOKEN"

# POST /openai/v1/chat/completions (body via stdin; returns raw provider JSON)
echo '{"model":"gpt-4.1-mini","messages":[]}' \
  | tinyhumans inference create-chat-completion --token "$TINYHUMANS_TOKEN"

# Query params
tinyhumans inference list-models --query limit=5
```

## Raw escape hatch

```bash
tinyhumans swagger --base-url https://api.tinyhumans.ai
tinyhumans raw get /auth/me --token "$TINYHUMANS_TOKEN"
echo '{"email":"you@example.com"}' | tinyhumans raw post /auth/email/send-link
```

## Global flags

- `--base-url URL`
- `--token TOKEN`
- `--api-key KEY`
- `--admin-service-token TOKEN`
- `--raw` — return the full `{ success, data }` envelope instead of unwrapping
- `--json` — compact JSON output (default is pretty-printed)

## Environment defaults

- `TINYHUMANS_BASE_URL`
- `TINYHUMANS_TOKEN`
- `TINYHUMANS_API_KEY`
- `TINYHUMANS_ADMIN_SERVICE_TOKEN`

## Regenerating the command table

The subcommand table is generated from the typed API modules into
`src/cli-commands.ts` (run automatically by `build`):

```bash
pnpm --filter @tinyhumansai/sdk gen:cli
```
