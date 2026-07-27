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

print(client.api_keys.list())
print(client.medulla.list_sessions())
```

The SDK is dependency-free and uses the standard-library `urllib` stack.
Namespace clients cover the deployed public API, including API keys, budgets,
Medulla, OpenCompany, orchestration, file storage, and history rewards.
Successful response envelopes are unwrapped by default; pass
`unwrap_envelope=False` to a method or use `client.raw` when the full response
or a newly deployed route is needed.

Only user JWT and API-key credentials are first-class options. Administrative
routes and service-token headers are intentionally not exposed.
