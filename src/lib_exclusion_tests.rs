use super::*;

#[test]
fn every_admin_and_webhook_route_is_rejected_by_the_raw_transport_gate() {
    // Pinned so the list can only ever be reviewed upward. A regenerated
    // spec that stopped describing admin or webhook routes would otherwise
    // shrink this list and silently unblock them at the raw transport.
    //
    // 50 -> 49: the prior resync (from main's spend-policy PR) had synced
    // against a spec that documented four `*/spend-policy` routes
    // (`GET /spend-policy`, `PUT /spend-policy`,
    // `PUT /api-keys/{keyId}/spend-policy`, and the admin
    // `PATCH /admin/users/{userId}/spend-policy` counted here) that the
    // backend never actually implements — it only has `spend-caps`
    // (`src/routes/spendCaps.ts` et al. in the backend repo; a full-text
    // search for `spend-policy` there finds zero matches). Resyncing
    // against a fresh, accurate spec dropped the one phantom admin entry,
    // taking this count from 50 back down to 49. The other three
    // `spend-policy` routes were never in this admin/webhook list to begin
    // with, since only `/admin/**` and undocumented `/webhooks/**` routes
    // land here.
    //
    // 49 -> 51: the two service-token operations on
    // `/opencompany/instances/{slug}/inference-key`. The orchestrator calls
    // them with a shared secret no SDK user holds, so they are excluded
    // from the client and blocked at the raw transport.
    //
    // 51 -> 54: the three write operations on `/admin/blog-posts`, which
    // the admin dashboard drives with the admin service token. The two
    // public `/blog/posts` reads that arrived with them are ordinary
    // user-facing API and are exposed; only the authoring side is blocked.
    //
    // 54 -> 55: `POST /opencompany/instances/{slug}/usage`, which the
    // orchestrator calls to report how long each hosted company held its
    // memory so the platform can bill it. Service-token authenticated like
    // the two `inference-key` operations above, and excluded for the same
    // reason: no SDK consumer holds the shared secret, and a client method
    // for it would only ever produce a 401. The user-facing read of the
    // same data is `GET /opencompany/instances/usage`, which is exposed.
    //
    // 55 -> 56: `POST /admin/blog-images`, the multipart upload the
    // dashboard uses for a post's cover and body figures. Same service
    // token as the other blog writes, so it is blocked alongside them.
    //
    // 56 -> 58: two service-to-service callbacks (since removed; see -> 60).
    //
    // 58 -> 59: `PUT /opencompany/instances/{slug}/orchestrator`, the
    // orchestrator's own service-token-authenticated callback (same shape
    // as the two `inference-key` operations and `.../usage` above), added
    // alongside `POST /opencompany/instances/{slug}/usage`.
    //
    // 59 -> 61: `PUT` and `DELETE /opencompany/orchestrators/{id}/token`,
    // the fleet's service-token-authenticated orchestrator token
    // registration, first synced alongside the Gemini routes.
    // Note: This assertion reflects the count when synced against the
    // deployed OpenAPI spec. When the backend branch adds routes that
    // aren't yet deployed, the local count may differ; the RETAINED_UNEXPOSED_ROUTES
    // in sync-openapi.mjs preserves admin/webhook operations regardless.
    // -> 59: every `/internal/*` service callback leaves: they are no
    // longer in the published spec, so this public list must not name
    // them, and `is_structurally_unexposed` blocks the whole `/internal`
    // prefix instead (see `internal_routes_are_blocked_without_being_named`).
    // The telemetry ingestion endpoint (OTEL / Langfuse) does NOT belong
    // here: it takes a normal user bearer token, not a service token, so
    // it stays in PUBLIC_ROUTES.
    // 59 -> 64: the five `GET /admin/memory/*` reads (`overview`, `storage`,
    // `top-tenants`, `tenants/{userId}` and the new `model-usage`). The first
    // four were already on backend `main` but never synced here; admin-only,
    // so blocked at the raw transport and given no typed method.
    assert_eq!(UNEXPOSED_ROUTES.len(), 64);
    for (method, template) in UNEXPOSED_ROUTES {
        let concrete_path = template
            .split('/')
            .map(|segment| {
                if segment.starts_with('{') && segment.ends_with('}') {
                    "example"
                } else {
                    segment
                }
            })
            .collect::<Vec<_>>()
            .join("/");
        let method = Method::from_bytes(method.as_bytes()).unwrap();
        assert!(
            matches!(
                reject_unexposed_route(&method, &concrete_path),
                Err(Error::RouteNotExposed(_, _))
            ),
            "{} {template} was not blocked",
            method.as_str()
        );
    }
}

/// Service-to-service routes are undocumented, so they cannot appear in
/// the generated lists; the raw transport refuses the whole prefix.
#[test]
fn internal_routes_are_blocked_without_being_named() {
    assert!(!UNEXPOSED_ROUTES
        .iter()
        .any(|(_, p)| p.starts_with("/internal")));
    assert!(!PUBLIC_ROUTES
        .iter()
        .any(|(_, p)| p.starts_with("/internal")));
    for (method, path) in [
        (Method::GET, "/internal/anything"),
        (Method::POST, "/internal/some/deeper/route"),
        (Method::DELETE, "/internal"),
    ] {
        assert!(
            matches!(
                reject_unexposed_route(&method, path),
                Err(Error::RouteNotExposed(_, _))
            ),
            "{method} {path} was not blocked"
        );
    }
    // A segment merely named `internal` further down is not the prefix.
    assert!(!is_structurally_unexposed(&Method::GET, "/teams/internal"));
}

/// Team-scoped operations gated by the team-admin *role* are not platform
/// administration and must stay reachable. Their OpenAPI summaries read
/// "(admin only)", which the generator's admin heuristic would otherwise
/// exclude — that broke OpenHuman's `team_remove_member`.
#[test]
fn team_role_gated_operations_are_not_treated_as_platform_admin() {
    for (method, path) in [
        ("PUT", "/teams/{teamId}"),
        ("DELETE", "/teams/{teamId}/members/{userId}"),
        ("PUT", "/teams/{teamId}/members/{userId}/role"),
    ] {
        assert!(
            !UNEXPOSED_ROUTES.contains(&(method, path)),
            "{method} {path} is team-role gated, not platform-admin"
        );
    }
}

/// Genuine platform-administration operations stay blocked, including the
/// ones outside an `/admin` path segment that only their summary marks.
#[test]
fn platform_admin_operations_remain_blocked() {
    for (method, path) in [
        ("POST", "/coupons/admin"),
        ("GET", "/coupons/admin"),
        ("PATCH", "/feedback/{id}/status"),
        ("POST", "/invite/campaign"),
        ("DELETE", "/invite/campaign/{codeId}"),
        ("POST", "/agent-integrations/composio/toolkits/refresh"),
    ] {
        assert!(
            UNEXPOSED_ROUTES.contains(&(method, path)),
            "{method} {path} is platform-admin and must stay blocked"
        );
    }
}

/// The blocked set covers webhook *receivers* only. `/webhooks/core*` is
/// user-owned tunnel CRUD — bearer-authenticated, user-facing, and driven
/// by OpenHuman — so it must stay reachable even though it shares the
/// `/webhooks` prefix with the receivers around it.
#[test]
fn webhook_tunnel_crud_is_not_in_the_blocked_set() {
    for (method, path) in [
        ("GET", "/webhooks/core"),
        ("POST", "/webhooks/core"),
        ("GET", "/webhooks/core/{id}"),
        ("PATCH", "/webhooks/core/{id}"),
        ("DELETE", "/webhooks/core/{id}"),
        ("GET", "/webhooks/core/bandwidth"),
    ] {
        assert!(
            !UNEXPOSED_ROUTES.contains(&(method, path)),
            "{method} {path} is user-facing and must not be blocked"
        );
    }
}

#[test]
fn future_structurally_private_routes_are_rejected_without_regeneration() {
    for (method, path) in [
        (Method::POST, "/admin/future-operation"),
        (Method::POST, "/webhooks/future-provider"),
    ] {
        assert!(
            matches!(
                reject_unexposed_route(&method, path),
                Err(Error::RouteNotExposed(_, _))
            ),
            "{} {path} was not blocked",
            method.as_str()
        );
    }
}

#[test]
fn generated_public_webhook_routes_pass_the_structural_guard() {
    for (method, path) in [
        (Method::GET, "/webhooks/core"),
        (Method::GET, "/webhooks/core/example"),
        (Method::GET, "/webhooks/core/bandwidth"),
    ] {
        assert!(
            reject_unexposed_route(&method, path).is_ok(),
            "{} {path} was blocked",
            method.as_str()
        );
    }
}
