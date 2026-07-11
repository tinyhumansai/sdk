# CLI

The TypeScript package ships the `tinyhumans` command.

```bash
pnpm --filter @tinyhumansai/sdk exec tinyhumans health --base-url https://api.tinyhumans.ai
pnpm --filter @tinyhumansai/sdk exec tinyhumans swagger --base-url https://api.tinyhumans.ai
pnpm --filter @tinyhumansai/sdk exec tinyhumans get /auth/me --base-url https://api.tinyhumans.ai --token "$TINYHUMANS_TOKEN"
echo '{"model":"gpt-4.1-mini","messages":[]}' | pnpm --filter @tinyhumansai/sdk exec tinyhumans post /openai/v1/chat/completions --token "$TINYHUMANS_TOKEN"
```

Environment defaults:

- `TINYHUMANS_BASE_URL`
- `TINYHUMANS_TOKEN`
- `TINYHUMANS_API_KEY`
- `TINYHUMANS_ADMIN_SERVICE_TOKEN`
