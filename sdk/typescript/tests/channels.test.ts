import { describe, expect, it } from "vitest";

import { ChannelsApi } from "../src/api/channels.js";
import { mockClient } from "./helpers.js";

describe("ChannelsApi", () => {
  it("sends a message with a JSON body", async () => {
    const { http, last } = mockClient({ data: { messageId: 42 } });
    const api = new ChannelsApi(http);

    const result = await api.sendMessage("telegram", { text: "hi", parseMode: "markdown" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/channels/telegram/messages");
    expect(call.body).toEqual({ text: "hi", parseMode: "markdown" });
    expect(result).toEqual({ messageId: 42 });
  });

  it("deletes a message with encoded path params", async () => {
    const { http, last } = mockClient({ data: { deleted: true } });
    const api = new ChannelsApi(http);

    await api.deleteMessage("discord", "msg 1");

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/channels/discord/messages/msg%201");
  });

  it("adds a reaction with a JSON body", async () => {
    const { http, last } = mockClient({ data: { ok: true } });
    const api = new ChannelsApi(http);

    await api.addReaction("telegram", { messageId: 7, emoji: "👍" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/channels/telegram/reactions");
    expect(call.body).toEqual({ messageId: 7, emoji: "👍" });
  });

  it("creates a thread", async () => {
    const { http, last } = mockClient({ data: { id: "thr_1" } });
    const api = new ChannelsApi(http);

    await api.createThread("discord", { title: "Support" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/channels/discord/threads");
    expect(call.body).toEqual({ title: "Support" });
  });

  it("lists threads with a query parameter", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new ChannelsApi(http);

    await api.listThreads("telegram", { active: true });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/channels/telegram/threads");
    expect(call.query.active).toEqual(["true"]);
  });

  it("updates a thread's status", async () => {
    const { http, last } = mockClient({ data: { id: "thr_1", status: "closed" } });
    const api = new ChannelsApi(http);

    await api.updateThread("discord", "thr_1", { action: "close" });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/channels/discord/threads/thr_1");
    expect(call.body).toEqual({ action: "close" });
  });

  it("sends a typing indicator", async () => {
    const { http, last } = mockClient({ data: { dispatched: true } });
    const api = new ChannelsApi(http);

    await api.sendTyping("telegram", { chatId: 99 });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/channels/telegram/typing");
    expect(call.body).toEqual({ chatId: 99 });
  });
});
