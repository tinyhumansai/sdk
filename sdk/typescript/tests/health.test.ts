import { describe, expect, it } from "vitest";

import { HealthApi } from "../src/api/health.js";
import { mockClient } from "./helpers.js";

describe("HealthApi", () => {
  it("checks the health endpoint at the root path", async () => {
    const { http, last } = mockClient({ data: { online: true, uptime: 1 } });
    const api = new HealthApi(http);

    await api.check();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/");
  });
});
