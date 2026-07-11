# Examples

## TypeScript

```ts
import { TinyHumansClient } from "@tinyhumansai/sdk";

const client = new TinyHumansClient({
  baseUrl: process.env.TINYHUMANS_BASE_URL ?? "https://api.tinyhumans.ai",
  token: process.env.TINYHUMANS_TOKEN,
});

console.log(await client.health());
console.log(await client.inference.get("/v1/models"));
```

## Python

```python
import os
from tinyhumans import TinyHumansClient

client = TinyHumansClient(
    base_url=os.getenv("TINYHUMANS_BASE_URL", "https://api.tinyhumans.ai"),
    token=os.getenv("TINYHUMANS_TOKEN"),
)

print(client.health())
print(client.inference.get("/v1/models"))
```

## Rust

```rust
use tinyhumans_sdk::TinyHumansClient;

# async fn run() -> Result<(), tinyhumans_sdk::Error> {
let client = TinyHumansClient::new(
    std::env::var("TINYHUMANS_BASE_URL").unwrap_or_else(|_| "https://api.tinyhumans.ai".into()),
).with_token(std::env::var("TINYHUMANS_TOKEN").ok());

println!("{:?}", client.health().await?);
println!("{:?}", client.inference().get("/v1/models").await?);
# Ok(())
# }
```
