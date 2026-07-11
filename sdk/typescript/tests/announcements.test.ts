import { describe, expect, it } from "vitest";

import { AnnouncementsApi } from "../src/api/announcements.js";
import { mockClient } from "./helpers.js";

describe("AnnouncementsApi", () => {
  it("gets the latest announcement", async () => {
    const { http, last } = mockClient({ data: { id: "a_1", title: "Hi" } });
    const api = new AnnouncementsApi(http);

    const result = await api.getLatestAnnouncements();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/announcements/latest");
    expect(result).toEqual({ id: "a_1", title: "Hi" });
  });
});
