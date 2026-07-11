# Examples

## TypeScript

```ts
import { TinyHumansClient } from "@tinyhumansai/sdk";

const client = new TinyHumansClient({
  baseUrl: process.env.TINYHUMANS_BASE_URL ?? "https://api.tinyhumans.ai",
  token: process.env.TINYHUMANS_TOKEN,
});

console.log(await client.health.check());
console.log(await client.inference.listModels());
// Typed methods per namespace, e.g.:
await client.auth.sendEmailLink({ email: "you@example.com" });
await client.payments.getCreditBalance();
```

## Python

```python
import os
from tinyhumans import TinyHumansClient

client = TinyHumansClient(
    base_url=os.getenv("TINYHUMANS_BASE_URL", "https://api.tinyhumans.ai"),
    token=os.getenv("TINYHUMANS_TOKEN"),
)

print(client.health.check())
print(client.inference.list_models())
# Typed methods per namespace, e.g.:
client.auth.send_email_link({"email": "you@example.com"})
print(client.payments.get_credit_balance())
```

## Rust

```rust
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new(
    std::env::var("TINYHUMANS_BASE_URL").unwrap_or_else(|_| "https://api.tinyhumans.ai".into()),
).with_token(std::env::var("TINYHUMANS_TOKEN").ok());

println!("{:?}", client.health().check().await?);
println!("{:?}", client.inference().list_models().await?);
# Ok(())
# }
```
