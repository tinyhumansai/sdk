# Python SDK

```bash
pip install tinyhumans
```

```python
from tinyhumans import TinyHumansClient

client = TinyHumansClient(
    base_url="https://api.tinyhumans.ai",
    token="...",
)

print(client.auth.get("/me"))
print(client.inference.get("/v1/models"))
```

The Python SDK is dependency-free for the first scaffold and uses the standard
library `urllib` stack. It exposes the same namespace clients as the TypeScript
SDK plus `client.raw` for newly deployed routes.
