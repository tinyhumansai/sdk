import { describe, expect, it } from "vitest";

import { TinyHumansClient } from "../src/client.js";
import { HttpClient } from "../src/http.js";

function stubFetch(calls: { url: string; init?: RequestInit }[]): typeof globalThis.fetch {
  return (async (url: string | URL, init?: RequestInit) => {
    calls.push({ url: url.toString(), init });
    return new Response(JSON.stringify({ openapi: "3.0.0" }), {
      status: 200,
      headers: { "content-type": "application/json" },
    });
  }) as typeof globalThis.fetch;
}

describe("TinyHumansClient", () => {
  it("exposes a raw HttpClient escape hatch", () => {
    const client = new TinyHumansClient({ baseUrl: "https://api.tinyhumans.ai" });
    expect(client.raw).toBeInstanceOf(HttpClient);
  });

  it("exposes namespace clients that are objects with methods", () => {
    const client = new TinyHumansClient({ baseUrl: "https://api.tinyhumans.ai" });

    for (const ns of [
      "agentIntegrations",
      "announcements",
      "auth",
      "channels",
      "coupons",
      "feedback",
      "health",
      "inference",
      "investors",
      "invite",
      "mascots",
      "payments",
      "redirect",
      "referral",
      "rewards",
      "teams",
      "webhooks",
    ] as const) {
      const value = (client as unknown as Record<string, unknown>)[ns];
      expect(value, ns).toBeTypeOf("object");
      expect(value, ns).not.toBeNull();
    }

    expect(typeof client.auth.me).toBe("function");
    expect(typeof client.teams.updateTeam).toBe("function");
  });

  it("swagger() requests /swagger.json without unwrapping the envelope", async () => {
    const calls: { url: string; init?: RequestInit }[] = [];
    const client = new TinyHumansClient({
      baseUrl: "https://api.tinyhumans.ai",
      fetch: stubFetch(calls),
    });

    const result = await client.swagger();

    expect(calls[0]!.url).toBe("https://api.tinyhumans.ai/swagger.json");
    expect(calls[0]!.init?.method).toBe("GET");
    // Not unwrapped: the full document is returned even though it lacks an envelope.
    expect(result).toEqual({ openapi: "3.0.0" });
  });
});
