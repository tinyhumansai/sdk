import { describe, expect, it } from "vitest";

import { RedirectApi } from "../src/api/redirect.js";
import { mockClient } from "./helpers.js";

describe("RedirectApi", () => {
  it("resolves a short link code, encoding the path", async () => {
    const { http, last } = mockClient({ data: { url: "https://example.com" } });
    const api = new RedirectApi(http);

    await api.resolveRedirect("ab cd");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/r/ab%20cd");
  });
});
