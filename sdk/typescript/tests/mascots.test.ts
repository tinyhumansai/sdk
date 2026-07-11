import { describe, expect, it } from "vitest";

import { MascotsApi } from "../src/api/mascots.js";
import { mockClient } from "./helpers.js";

describe("MascotsApi", () => {
  it("lists mascots", async () => {
    const { http, last } = mockClient({ data: [{ id: "robo" }] });
    const api = new MascotsApi(http);

    const result = await api.listMascots();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/mascots");
    expect(result).toEqual([{ id: "robo" }]);
  });

  it("gets the demo page", async () => {
    const { http, last } = mockClient({ text: "<html></html>" });
    const api = new MascotsApi(http);

    await api.getDemo();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/mascots/demo");
  });

  it("joins a meeting with a JSON body", async () => {
    const { http, last } = mockClient({ data: { botId: "bot_1" } });
    const api = new MascotsApi(http);

    await api.joinMeeting({ meetUrl: "https://meet.google.com/abc", muted: true });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/mascots/join-meeting");
    expect(call.body).toEqual({ meetUrl: "https://meet.google.com/abc", muted: true });
  });

  it("lists meetings with a limit query parameter", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new MascotsApi(http);

    await api.listMeetings({ limit: 5 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/mascots/meetings");
    expect(call.query.limit).toEqual(["5"]);
  });

  it("gets a mascot manifest with an encoded id", async () => {
    const { http, last } = mockClient({ data: { id: "robo/1" } });
    const api = new MascotsApi(http);

    await api.getMascot("robo/1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/mascots/robo%2F1");
  });

  it("downloads the Rive file with an encoded id", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new MascotsApi(http);

    await api.getMascotRiv("robo/1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/mascots/robo%2F1/riv");
  });
});
