#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import process from "node:process";

const SPEC_URL = "https://api.tinyhumans.ai/swagger.json";
const MANIFEST_PATH = resolve("api/tinyhumans.backend.json");
const RUST_ROUTES_PATH = resolve("src/generated_public_routes.rs");
const HTTP_METHODS = new Set([
  "delete",
  "get",
  "head",
  "options",
  "patch",
  "post",
  "put",
  "trace",
]);

const namespaceOverrides = new Map([
  ["agent-integrations", "agentIntegrations"],
  ["api-keys", "apiKeys"],
  ["openai", "inference"],
  ["opencompany", "openCompany"],
  ["r", "redirect"],
]);

// Public backend calls implemented and exercised by OpenHuman but not yet
// described by the deployed Swagger document.
const SUPPLEMENTAL_PUBLIC_OPERATIONS = [
  ["GET", "/agent-integrations/composio/github/repos"],
  ["POST", "/agent-integrations/tinyfish/agent/run"],
  ["POST", "/agent-integrations/tinyfish/fetch"],
  ["POST", "/agent-integrations/tinyfish/search"],
  ["GET", "/orchestration/v1/steering"],
  ["PUT", "/teams/{teamId}"],
  ["DELETE", "/teams/{teamId}/members/{userId}"],
  ["PUT", "/teams/{teamId}/members/{userId}/role"],
  ["GET", "/webhooks/core"],
  ["POST", "/webhooks/core"],
  ["DELETE", "/webhooks/core/{id}"],
  ["GET", "/webhooks/core/{id}"],
  ["PATCH", "/webhooks/core/{id}"],
  ["GET", "/webhooks/core/bandwidth"],
];

// Admin and webhook operations the SDK must keep unreachable through both the
// typed and the raw APIs. The backend now serves a PRE-FILTERED public
// Swagger document (`src/config/swagger.ts` -> `publicSwaggerSpec`), so these
// operations no longer appear in the deployed spec at all and can no longer be
// derived from it. They are declared here so regeneration retains the filter
// instead of silently emptying the denylist and opening the raw transport.
const RETAINED_UNEXPOSED_ROUTES = [
  ["POST", "/admin/announcements"],
  ["DELETE", "/admin/announcements/{announcementId}"],
  ["PATCH", "/admin/announcements/{announcementId}"],
  ["POST", "/admin/coupons"],
  ["DELETE", "/admin/coupons/{couponId}"],
  ["PATCH", "/admin/coupons/{couponId}"],
  ["POST", "/admin/coupons/bulk"],
  ["PATCH", "/admin/feedback/triage/{feedbackId}"],
  ["POST", "/admin/feedback/triage/{feedbackId}/approve"],
  ["POST", "/admin/feedback/triage/{feedbackId}/merge"],
  ["POST", "/admin/feedback/triage/{feedbackId}/reject"],
  ["POST", "/admin/feedback/triage/{feedbackId}/reprocess"],
  ["POST", "/admin/users/{userId}/credits"],
  ["PATCH", "/admin/users/{userId}/medulla-access"],
  ["DELETE", "/admin/users/{userId}/subscription"],
  ["POST", "/admin/users/{userId}/subscription"],
  ["POST", "/admin/users/credits/bulk"],
  ["POST", "/agent-integrations/composio/toolkits/refresh"],
  ["POST", "/agent-integrations/twilio/webhooks/incoming-call/{userId}"],
  ["POST", "/agent-integrations/twilio/webhooks/status/{userId}"],
  ["GET", "/coupons/admin"],
  ["POST", "/coupons/admin"],
  ["DELETE", "/coupons/admin/{couponId}"],
  ["PATCH", "/feedback/{id}/status"],
  ["GET", "/feedback/admin/triage"],
  ["GET", "/feedback/admin/triage/{id}"],
  ["POST", "/feedback/admin/triage/{id}/approve"],
  ["PATCH", "/feedback/admin/triage/{id}/draft"],
  ["POST", "/feedback/admin/triage/{id}/merge"],
  ["POST", "/feedback/admin/triage/{id}/reject"],
  ["POST", "/feedback/admin/triage/{id}/reprocess"],
  ["GET", "/invite/campaign"],
  ["POST", "/invite/campaign"],
  ["DELETE", "/invite/campaign/{codeId}"],
  ["POST", "/webhooks/composio"],
  ["POST", "/webhooks/discord"],
  ["POST", "/webhooks/github"],
  ["POST", "/webhooks/ingress/{uuid}"],
  ["POST", "/webhooks/ingress/{uuid}/{path}"],
  ["POST", "/webhooks/payments/coinbase"],
  ["POST", "/webhooks/payments/stripe"],
  ["POST", "/webhooks/sentry"],
  ["POST", "/webhooks/telegram"],
  ["POST", "/webhooks/telegram/managed/{botId}"],
];

function parseArgs(argv) {
  const options = { check: false, input: undefined };
  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--check") {
      options.check = true;
    } else if (argument === "--input") {
      options.input = argv[++index];
      if (!options.input) throw new Error("--input requires a file path");
    } else {
      throw new Error(`Unknown argument: ${argument}`);
    }
  }
  return options;
}

// Operations gated by a role *within a resource the caller belongs to*, not by
// platform administrator rights.
//
// Their OpenAPI summaries read "(admin only)", which the heuristic below would
// otherwise treat as platform-admin and exclude. But the "admin" here is the
// team-admin role: any user who creates a team holds it, the routes take an
// ordinary `bearerAuth` user token, they are scoped to a team the caller is a
// member of, and their 403s read "Only admins can remove members" — meaning
// admins *of that team*. Platform-administrator operations live under an
// `/admin` path segment and are excluded by path above.
//
// Excluding these broke OpenHuman's team-management UI: `team_remove_member`
// failed with "route is intentionally not exposed by the SDK".
const TEAM_ROLE_GATED_OPERATIONS = new Set([
  "PUT /teams/{teamId}",
  "DELETE /teams/{teamId}/members/{userId}",
  "PUT /teams/{teamId}/members/{userId}/role",
]);

function isAdminOperation(path, operation, method) {
  if (TEAM_ROLE_GATED_OPERATIONS.has(`${method.toUpperCase()} ${path}`)) {
    return false;
  }
  if (
    path === "/admin" ||
    path.startsWith("/admin/") ||
    path === "/coupons/admin" ||
    path.startsWith("/coupons/admin/") ||
    path === "/feedback/admin" ||
    path.startsWith("/feedback/admin/")
  ) {
    return true;
  }

  const summary = operation.summary ?? "";
  const description = operation.description ?? "";
  const forbiddenDescription = operation.responses?.["403"]?.description ?? "";
  return (
    /\(admin\)/i.test(summary) ||
    /\badmin(?:istrator)?(?:\s+access)?\s+only\b/i.test(
      `${summary} ${description} ${forbiddenDescription}`,
    )
  );
}

// Exclude webhook *receivers* — the endpoints providers call into (Stripe,
// Telegram, Discord, GitHub, Composio, Coinbase, Sentry, Twilio, and the
// tunnel ingress routes). They are authenticated by provider signature rather
// than by a user bearer token, so an SDK caller has nothing to send and no
// reason to invoke them.
//
// Matching on the path segment alone also swept in `/webhooks/core*`, the
// user-owned webhook *tunnel* CRUD surface — list/create/read/update/delete a
// tunnel plus its bandwidth budget. Those are ordinary authenticated
// user-facing operations that happen to live under `/webhooks`, and OpenHuman
// drives all six of them from its `webhooks` RPC namespace. Blocking them left
// that feature with no SDK path at all, not even through the raw escape hatch,
// which `reject_unexposed_route` gates on this same list.
//
// `bearerAuth` is the discriminator rather than a hardcoded `/webhooks/core`
// allowlist: it is the property that actually distinguishes the two families,
// so a future user-facing webhook-management route is classified correctly
// without another edit here.
function isWebhookOperation(path, operation) {
  if (!isWebhookPath(path)) return false;
  return operationAuth(operation) !== "bearer";
}

function isWebhookPath(path) {
  return path.split("/").includes("webhooks");
}

function namespaceFor(path) {
  if (path === "/") return "health";
  const segment = path.split("/")[1];
  return namespaceOverrides.get(segment) ?? segment.replace(
    /-([a-z])/g,
    (_, letter) => letter.toUpperCase(),
  );
}

function basePathFor(namespace, path) {
  if (namespace === "health") return "";
  if (namespace === "inference") return "/openai";
  if (namespace === "redirect") return "/r";
  return `/${path.split("/")[1]}`;
}

function operationAuth(operation) {
  const security = operation.security ?? [];
  return security.some((entry) => Object.hasOwn(entry, "bearerAuth"))
    ? "bearer"
    : "none";
}

function namespaceAuth(operations) {
  const values = new Set(operations.map(({ operation }) => operationAuth(operation)));
  return values.size === 1 ? [...values][0] : "mixed";
}

function buildManifest(spec) {
  const publicOperations = [];
  const excludedOperations = [];
  let totalOperationCount = 0;

  for (const path of Object.keys(spec.paths ?? {}).sort()) {
    const pathItem = spec.paths[path];
    for (const method of Object.keys(pathItem).sort()) {
      if (!HTTP_METHODS.has(method)) continue;
      totalOperationCount += 1;
      const operation = pathItem[method];
      if (isAdminOperation(path, operation, method)) {
        excludedOperations.push({ method: method.toUpperCase(), path });
        continue;
      }
      if (isWebhookOperation(path, operation)) {
        excludedOperations.push({ method: method.toUpperCase(), path });
        continue;
      }
      publicOperations.push({
        method: method.toUpperCase(),
        namespace: namespaceFor(path),
        operation,
        path,
      });
    }
  }

  // Retain the declared denylist on top of whatever the spec still describes, so
  // a spec that stops documenting its admin/webhook surface cannot widen the SDK.
  for (const [method, path] of RETAINED_UNEXPOSED_ROUTES) {
    if (excludedOperations.some((entry) => entry.method === method && entry.path === path)) {
      continue;
    }
    excludedOperations.push({ method, path });
  }
  const excludedWebhookOperationCount = excludedOperations.filter(({ path }) =>
    isWebhookPath(path)
  ).length;
  const excludedAdminOperationCount = excludedOperations.length -
    excludedWebhookOperationCount;

  for (const [method, path] of SUPPLEMENTAL_PUBLIC_OPERATIONS) {
    publicOperations.push({
      method,
      namespace: namespaceFor(path),
      operation: {
        summary: "Public operation implemented by the OpenHuman backend client",
        security: [{ bearerAuth: [] }],
        tags: ["OpenHuman parity"],
      },
      path,
    });
  }

  const grouped = Map.groupBy(publicOperations, ({ namespace }) => namespace);
  const namespaces = [...grouped.entries()]
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, operations]) => ({
      name,
      basePath: basePathFor(name, operations[0].path),
      auth: namespaceAuth(operations),
      operationCount: operations.length,
      tags: [...new Set(operations.flatMap(({ operation }) => operation.tags ?? []))].sort(),
      routes: operations
        .map(({ method, path }) => `${method} ${path}`)
        .sort(),
    }));

  return {
    excludedOperations,
    manifest: {
    $schema: "https://json-schema.org/draft/2020-12/schema",
    name: "tinyhumans-backend",
    version: "0.2.0",
    description:
      "TinyHumans public backend namespace manifest derived from the deployed Swagger/OpenAPI contract. Administrative and webhook operations are intentionally excluded.",
    source: {
      type: "openapi",
      url: SPEC_URL,
      title: spec.info?.title,
      version: spec.info?.version,
      pathCount: Object.keys(spec.paths ?? {}).length,
      totalOperationCount,
      operationCount: publicOperations.length,
      supplementalOperationCount: SUPPLEMENTAL_PUBLIC_OPERATIONS.length,
      excludedAdminOperationCount,
      excludedWebhookOperationCount,
      servers: (spec.servers ?? []).map(({ url }) => url),
    },
    envelope: {
      success: "boolean",
      data: "any",
      error: "string | object",
    },
    auth: {
      bearerToken: "Authorization: Bearer <token>",
      apiKey: "x-api-key: <key>",
    },
    namespaces,
    },
  };
}

function rustRouteEntries(routes) {
  return routes
    .map(({ method, path }) => `    ("${method}", "${path}"),`)
    .join("\n");
}

function buildRustRoutes(manifest, excludedOperations) {
  const routes = manifest.namespaces
    .flatMap(({ routes }) => routes)
    .map((route) => {
      const separator = route.indexOf(" ");
      return [route.slice(0, separator), route.slice(separator + 1)];
    })
    .sort(([leftMethod, leftPath], [rightMethod, rightPath]) =>
      leftPath.localeCompare(rightPath) || leftMethod.localeCompare(rightMethod)
    );
  const entries = rustRouteEntries(
    routes.map(([method, path]) => ({ method, path })),
  );
  const excludedEntries = rustRouteEntries(
    excludedOperations.sort(
      (left, right) =>
        left.path.localeCompare(right.path) ||
        left.method.localeCompare(right.method),
    ),
  );
  return `// Generated by scripts/sync-openapi.mjs. Do not edit.
/// Every public, non-admin, non-webhook route exposed by the deployed backend contract.
#[rustfmt::skip]
pub const PUBLIC_ROUTES: &[(&str, &str)] = &[
${entries}
];

/// Route templates intentionally unavailable through both typed and raw SDK APIs.
#[rustfmt::skip]
pub(crate) const UNEXPOSED_ROUTES: &[(&str, &str)] = &[
${excludedEntries}
];
`;
}

async function loadSpec(input) {
  if (input) return JSON.parse(await readFile(resolve(input), "utf8"));
  const response = await fetch(SPEC_URL);
  if (!response.ok) {
    throw new Error(`Failed to fetch ${SPEC_URL}: HTTP ${response.status}`);
  }
  return response.json();
}

const options = parseArgs(process.argv.slice(2));
const { manifest: manifestObject, excludedOperations } = buildManifest(
  await loadSpec(options.input),
);
const manifest = `${JSON.stringify(manifestObject, null, 2)}\n`;
const rustRoutes = buildRustRoutes(manifestObject, excludedOperations);

if (options.check) {
  const [currentManifest, currentRustRoutes] = await Promise.all([
    readFile(MANIFEST_PATH, "utf8"),
    readFile(RUST_ROUTES_PATH, "utf8"),
  ]);
  if (currentManifest !== manifest || currentRustRoutes !== rustRoutes) {
    console.error(
      "Generated OpenAPI surfaces are out of date; run node scripts/sync-openapi.mjs",
    );
    process.exitCode = 1;
  }
} else {
  await Promise.all([
    writeFile(MANIFEST_PATH, manifest),
    writeFile(RUST_ROUTES_PATH, rustRoutes),
  ]);
}
