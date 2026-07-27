#!/usr/bin/env node

import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import process from "node:process";

const SPEC_URL = "https://api.tinyhumans.ai/swagger.json";
const MANIFEST_PATH = resolve("api/tinyhumans.backend.json");
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

function isAdminOperation(path, operation) {
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
  let totalOperationCount = 0;
  let excludedAdminOperationCount = 0;

  for (const path of Object.keys(spec.paths ?? {}).sort()) {
    const pathItem = spec.paths[path];
    for (const method of Object.keys(pathItem).sort()) {
      if (!HTTP_METHODS.has(method)) continue;
      totalOperationCount += 1;
      const operation = pathItem[method];
      if (isAdminOperation(path, operation)) {
        excludedAdminOperationCount += 1;
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
    $schema: "https://json-schema.org/draft/2020-12/schema",
    name: "tinyhumans-backend",
    version: "0.2.0",
    description:
      "TinyHumans public backend namespace manifest derived from the deployed Swagger/OpenAPI contract. Administrative operations are intentionally excluded.",
    source: {
      type: "openapi",
      url: SPEC_URL,
      title: spec.info?.title,
      version: spec.info?.version,
      pathCount: Object.keys(spec.paths ?? {}).length,
      totalOperationCount,
      operationCount: publicOperations.length,
      excludedAdminOperationCount,
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
  };
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
const manifest = `${JSON.stringify(buildManifest(await loadSpec(options.input)), null, 2)}\n`;

if (options.check) {
  const current = await readFile(MANIFEST_PATH, "utf8");
  if (current !== manifest) {
    console.error(
      "api/tinyhumans.backend.json is out of date; run node scripts/sync-openapi.mjs",
    );
    process.exitCode = 1;
  }
} else {
  await writeFile(MANIFEST_PATH, manifest);
}
