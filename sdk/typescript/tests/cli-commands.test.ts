import { describe, expect, it } from "vitest";

import { COMMANDS } from "../src/cli-commands.js";
import { TinyHumansClient } from "../src/index.js";

const NAMESPACES = [
  "agentIntegrations",
  "apiKeys",
  "announcements",
  "auth",
  "budgets",
  "channels",
  "coupons",
  "feedback",
  "health",
  "inference",
  "invite",
  "mascots",
  "medulla",
  "openCompany",
  "orchestration",
  "payments",
  "redirect",
  "referral",
  "rewards",
  "teams",
  "webhooks",
];

describe("CLI command manifest", () => {
  it("covers every public namespace", () => {
    expect(COMMANDS).toHaveLength(201);
    const found = new Set(COMMANDS.map((c) => c.namespace));
    expect([...found].sort()).toEqual([...NAMESPACES].sort());
  });

  it("has a resolvable verb and path for every command", () => {
    for (const cmd of COMMANDS) {
      expect(cmd.verb).toMatch(/^(GET|POST|PUT|PATCH|DELETE)$/);
      expect(cmd.path.startsWith("/")).toBe(true);
      expect(cmd.command).toMatch(/^[a-z0-9-]+$/);
    }
  });

  it("orders params and marks at most one body/query each", () => {
    for (const cmd of COMMANDS) {
      const bodies = cmd.params.filter((p) => p.role === "body");
      const queries = cmd.params.filter((p) => p.role === "query");
      expect(bodies.length).toBeLessThanOrEqual(1);
      expect(queries.length).toBeLessThanOrEqual(1);
      // Every :param in the path is a positional param on the method.
      const positional = new Set(
        cmd.params.filter((p) => p.role === "positional").map((p) => p.name),
      );
      for (const pp of cmd.pathParams) expect(positional.has(pp)).toBe(true);
    }
  });

  it("maps each command to an actual method on its namespace client", () => {
    const client = new TinyHumansClient({ baseUrl: "https://api.tinyhumans.ai" });
    for (const cmd of COMMANDS) {
      const ns = (client as unknown as Record<string, Record<string, unknown>>)[cmd.namespace];
      expect(ns, `namespace ${cmd.namespace}`).toBeDefined();
      expect(typeof ns[cmd.method], `${cmd.namespace}.${cmd.method}`).toBe("function");
    }
  });

  it("flags inference commands as non-enveloped", () => {
    const inference = COMMANDS.filter((c) => c.namespace === "inference");
    expect(inference.length).toBeGreaterThan(0);
    expect(inference.every((c) => c.nonEnvelope)).toBe(true);
  });
});
