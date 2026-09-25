use std::collections::BTreeSet;

use serde_json::json;
use tinyhumans_sdk::api::api_keys::{ApiKeyScope, CreatableApiKeyScope, CreateApiKeyRequest};
use tinyhumans_sdk::generated_public_routes::PUBLIC_ROUTES;
use tinyhumans_sdk::{Error, TinyHumansClient};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ok() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"success": true, "data": {"ok": true}}))
}

#[tokio::test]
async fn typed_api_key_request_uses_openapi_field_names() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api-keys"))
        .and(body_json(
            json!({"name":"CI","scopes":["inference"],"allowedIps":["10.0.0.0/8"]}),
        ))
        .respond_with(ok())
        .mount(&server)
        .await;
    let request = CreateApiKeyRequest {
        name: "CI".into(),
        scopes: vec![CreatableApiKeyScope::Inference],
        allowed_ips: vec!["10.0.0.0/8".into()],
        expires_at: None,
    };
    TinyHumansClient::new(server.uri())
        .api_keys()
        .create(&request)
        .await
        .unwrap();
}

#[test]
fn creatable_api_key_scope_excludes_the_machine_only_connections_scope() {
    // `POST /api-keys` rejects `connections` (it is granted automatically to
    // provisioned tenant origins by the `GET /auth/key` PKCE flow instead).
    // `CreateApiKeyRequest.scopes` uses `CreatableApiKeyScope`, which has no
    // `Connections` variant at all, so this can't compile back in by
    // accident; this asserts the wire values that *are* reachable stay in
    // sync with `ApiKeyScope` minus `connections`.
    let creatable_wire_values: BTreeSet<String> = [
        CreatableApiKeyScope::Inference,
        CreatableApiKeyScope::Voice,
        CreatableApiKeyScope::Search,
        CreatableApiKeyScope::Media,
        CreatableApiKeyScope::Storage,
        CreatableApiKeyScope::Meetings,
        CreatableApiKeyScope::Account,
        CreatableApiKeyScope::Companies,
    ]
    .iter()
    .map(|scope| match serde_json::to_value(scope).unwrap() {
        serde_json::Value::String(s) => s,
        other => panic!("expected a string, got {other:?}"),
    })
    .collect();
    assert_eq!(
        creatable_wire_values,
        BTreeSet::from(
            [
                "inference",
                "voice",
                "search",
                "media",
                "storage",
                "meetings",
                "account",
                "companies"
            ]
            .map(String::from)
        )
    );
    assert!(!creatable_wire_values.contains("connections"));
}

#[test]
fn create_api_key_rejects_the_machine_only_connections_scope() {
    // `POST /api-keys` rejects a `connections` mint outright (see
    // `HUMAN_MINTABLE_SCOPES` in the backend's `apiKey.ts`), so the SDK must
    // catch this client-side rather than let the request go out and fail.
    // `CreateApiKeyRequest.scopes` can't even hold `Connections`; the one
    // way to get there from a full `ApiKeyScope` (say, copied off a listed
    // key) is the fallible narrowing, which is where the error surfaces.
    let err = CreatableApiKeyScope::try_from(ApiKeyScope::Connections).unwrap_err();
    assert!(matches!(
        err,
        Error::ScopeNotCreatable(ApiKeyScope::Connections)
    ));
    // Every other scope narrows and widens back to itself.
    for scope in [
        ApiKeyScope::Inference,
        ApiKeyScope::Voice,
        ApiKeyScope::Search,
        ApiKeyScope::Media,
        ApiKeyScope::Storage,
        ApiKeyScope::Meetings,
        ApiKeyScope::Account,
        ApiKeyScope::Companies,
    ] {
        let creatable = CreatableApiKeyScope::try_from(scope.clone()).unwrap();
        assert_eq!(ApiKeyScope::from(creatable), scope);
    }
}

#[tokio::test]
async fn path_segments_are_encoded_on_typed_routes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/feedback/a%2Fb"))
        .respond_with(ok())
        .mount(&server)
        .await;
    TinyHumansClient::new(server.uri())
        .feedback()
        .get_feedback("a/b")
        .await
        .unwrap();
}

#[test]
fn generated_rust_routes_match_the_public_manifest() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../api/tinyhumans.backend.json")).unwrap();
    let manifest_routes = manifest["namespaces"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|namespace| namespace["routes"].as_array().unwrap())
        .map(|route| {
            let route = route.as_str().unwrap();
            route.split_once(' ').unwrap()
        })
        .collect::<BTreeSet<_>>();
    let rust_routes = PUBLIC_ROUTES.iter().copied().collect::<BTreeSet<_>>();

    // 227 -> 229: the two public blog reads, `GET /blog/posts` and
    // `GET /blog/posts/{slug}`. 229 -> 230: `GET /payments/summary`, the
    // authenticated billing summary used by clients to render account
    // credit state. 230 -> 234: the four `/auth/key*` grant routes (key
    // issuance for scoped API keys), picked up when resyncing against the
    // backend's teams-removal spec.
    //
    // 234 -> 237: the dashboard's usage reads —
    // `GET /opencompany/instances/usage`, `GET /payments/credits/ledger` and
    // `GET /payments/credits/ledger/export`.
    //
    // 237 -> 202: the `/medulla/v1/*` and `/orchestration/v1/*` families are
    // gone with the orchestration model. The backend's part in Medulla is now
    // the plan entitlement on `/auth/me`, apart from the authenticated empty
    // session list kept for older OpenHuman clients.
    // 202 -> 203: `GET /payments/credits/lots`, the caller's live credit lots
    // and their expiries (subscription credit no longer rolls over; top-ups
    // last a year).
    // 203 -> 204: `POST /opencompany/instances/{slug}/update`, the
    // owner-triggered "update to latest" for a hosted company.
    // 204 -> 206: OpenRouter System One plus its TypeSafe-compatible alias.
    // 206 -> 208: `GET /opencompany/companies`, the company-template catalog,
    // and `POST /opencompany/instances/{slug}/update`, the owner-triggered
    // "update to latest" — both are served by the backend `main` this syncs
    // against (sdk main had been generated from the deployed spec that predated
    // them).
    // 208 -> 211: the Gemini integration — `POST .../gemini/models/{model}/generate-content`,
    // `POST .../gemini/live/sessions` and `GET .../gemini/live/sessions/{sessionId}`.
    // 211 -> 212: the retired orchestration session-list compatibility route.
    assert_eq!(manifest["source"]["operationCount"], 212);
    // 14 -> 13: `GET /orchestration/v1/steering` left with that family.
    assert_eq!(manifest["source"]["supplementalOperationCount"], 13);
    // 37 -> 39: the two service-token operations on
    // `/opencompany/instances/{slug}/inference-key`. They are counted with the
    // admin exclusions because that tally is derived from `excludedOperations`,
    // which now holds every non-webhook exclusion including these.
    //
    // 39 -> 42: the three `/admin/blog-posts` writes that arrived with those
    // reads. Same change, opposite side of the line: the reads are ordinary
    // user-facing API, the writes take the admin service token.
    //
    // 42 -> 43: `POST /opencompany/instances/{slug}/usage`, the orchestrator's
    // runtime-billing report. It leaves `operationCount` untouched because a
    // service-token route never enters the public surface — it is excluded by
    // its security requirement rather than by name, so it lands here and
    // nowhere else. The user-facing read of the same data is
    // `GET /opencompany/instances/usage`, which is public.
    //
    // 43 -> 44: `POST /admin/blog-images`, the multipart upload behind a
    // post's cover and body figures. Same token as the other blog writes.
    //
    // 44 -> 46: `POST /internal/discord/link` and
    // `DELETE /internal/discord/link/{userId}`, the teeny Discord service's
    // account-link callbacks, gated by GUILD_SERVICE_TOKEN. Service-token
    // routes, so they land here and never in the public surface; the
    // user-facing half of that flow is `POST /auth/guild/link-token`.
    //
    // 46 -> 47: `PUT /opencompany/instances/{slug}/orchestrator`, the
    // orchestrator's own service-token callback (same shape as the two
    // `inference-key` operations and `.../usage` above).
    //
    // 47 -> 49: `PUT` and `DELETE /opencompany/orchestrators/{id}/token`, the
    // fleet's service-token orchestrator token registration, already on
    // backend `main` and first synced with the Gemini routes.
    assert_eq!(manifest["source"]["excludedAdminOperationCount"], 49);
    assert_eq!(manifest["source"]["excludedWebhookOperationCount"], 12);
    // 206 -> 208: the two new public opencompany routes above
    // (`GET /opencompany/companies` and `POST /opencompany/instances/{slug}/update`).
    // 208 -> 211: the three Gemini routes.
    // 211 -> 212: the retired orchestration session-list compatibility route.
    assert_eq!(rust_routes.len(), 212);
    assert_eq!(rust_routes, manifest_routes);
    assert!(rust_routes
        .iter()
        .all(|(_, path)| !path.split('/').any(|segment| segment == "admin")));
    // Webhook *receivers* stay out of the public surface: they are provider
    // callbacks authenticated by signature, so an SDK caller invoking one would
    // be forging provider traffic. The user-owned tunnel CRUD under
    // `/webhooks/core` is ordinary bearer-authenticated user-facing API and is
    // the one permitted exception under this prefix.
    assert!(rust_routes.iter().all(|(_, path)| {
        !path.split('/').any(|segment| segment == "webhooks")
            || *path == "/webhooks/core"
            || path.starts_with("/webhooks/core/")
    }));
}
