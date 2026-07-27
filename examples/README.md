# Rust examples

Run the live backend contract probe against production:

```bash
TINYHUMANS_E2E_BASE_URL=https://api.tinyhumans.ai \
  cargo run --example live_contract
```

The probe fetches health and Swagger through the Rust client and verifies that
a protected public route rejects an unauthenticated request.
