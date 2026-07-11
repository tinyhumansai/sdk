import { describe, expect, it } from "vitest";

import { FeedbackApi } from "../src/api/feedback.js";
import { mockClient } from "./helpers.js";

describe("FeedbackApi", () => {
  it("creates feedback with a JSON body", async () => {
    const { http, last } = mockClient({ data: { id: "fb_1" } });
    const api = new FeedbackApi(http);

    const result = await api.createFeedback({ type: "bug", title: "Crash", body: "It crashes" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/feedback");
    expect(call.body).toEqual({ type: "bug", title: "Crash", body: "It crashes" });
    expect(result).toEqual({ id: "fb_1" });
  });

  it("lists feedback with query parameters", async () => {
    const { http, last } = mockClient({ data: { items: [] } });
    const api = new FeedbackApi(http);

    await api.listFeedback({ type: "feature", sort: "hot", page: 2 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/feedback");
    expect(call.query.type).toEqual(["feature"]);
    expect(call.query.sort).toEqual(["hot"]);
    expect(call.query.page).toEqual(["2"]);
  });

  it("gets a feedback item with an encoded path param", async () => {
    const { http, last } = mockClient({ data: { id: "fb 1" } });
    const api = new FeedbackApi(http);

    await api.getFeedback("fb 1");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/feedback/fb%201");
  });

  it("comments on a feedback item", async () => {
    const { http, last } = mockClient({ status: 201, data: { id: "c_1" } });
    const api = new FeedbackApi(http);

    await api.commentFeedback("fb_1", { body: "Nice" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/feedback/fb_1/comments");
    expect(call.body).toEqual({ body: "Nice" });
  });

  it("updates a feedback item's status", async () => {
    const { http, last } = mockClient({ data: { id: "fb_1", status: "planned" } });
    const api = new FeedbackApi(http);

    await api.updateFeedbackStatus("fb_1", { status: "planned" });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/feedback/fb_1/status");
    expect(call.body).toEqual({ status: "planned" });
  });

  it("votes on a feedback item", async () => {
    const { http, last } = mockClient({ data: { up: 3, down: 0 } });
    const api = new FeedbackApi(http);

    await api.voteFeedback("fb_1", { value: 1 });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/feedback/fb_1/vote");
    expect(call.body).toEqual({ value: 1 });
  });
});
