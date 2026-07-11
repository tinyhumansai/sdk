# Authentication

The TinyHumans backend uses explicit HTTP credentials. The SDKs do not rely on
cookies.

## User or Agent Bearer Token

Pass a JWT or issued token as a bearer credential:

```text
Authorization: Bearer <token>
```

Each SDK accepts `token` on client construction.

## API Key

Some machine routes support:

```text
x-api-key: <key>
```

Each SDK accepts `apiKey` or `api_key`.

## Admin Service Token

Service-to-service admin write routes use:

```text
x-admin-service-token: <token>
```

Use this only in trusted server environments.

## Client Version Headers

The backend records version headers such as `x-sdk-version`,
`x-tauri-version`, `x-core-version`, and `x-ios-version`. The SDKs set
`x-sdk-client: tinyhumans-sdk` and allow callers to add extra headers.
