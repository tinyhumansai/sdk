import { describe, expect, it } from "vitest";

import { AuthApi } from "../src/api/auth.js";
import { mockClient } from "./helpers.js";

describe("AuthApi", () => {
  it("sends a magic link with a JSON body", async () => {
    const { http, last } = mockClient({ data: { sent: true } });
    const api = new AuthApi(http);

    const result = await api.sendEmailLink({ email: "a@b.com" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/auth/email/send-link");
    expect(call.body).toEqual({ email: "a@b.com" });
    expect(result).toEqual({ sent: true });
  });

  it("verifies email with the token as a query parameter", async () => {
    const { http, last } = mockClient({ data: { ok: true } });
    const api = new AuthApi(http);

    await api.verifyEmail("tok_123");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/auth/email/verify");
    expect(call.query.token).toEqual(["tok_123"]);
  });

  it("unwraps the current user envelope", async () => {
    const { http } = mockClient({ data: { id: "user_1", email: "a@b.com" } });
    const api = new AuthApi(http);

    const user = await api.me();

    expect(user).toEqual({ id: "user_1", email: "a@b.com" });
  });

  it("encodes path segments for channel link tokens", async () => {
    const { http, last } = mockClient({ data: { token: "lt_1" } });
    const api = new AuthApi(http);

    await api.createChannelLinkToken("telegram");

    expect(last().path).toBe("/auth/channels/telegram/link-token");
    expect(last().method).toBe("POST");
  });

  it("passes OAuth connect query parameters", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new AuthApi(http);

    await api.oauthConnect("google", { skillId: "skill_9" });

    expect(last().path).toBe("/auth/google/connect");
    expect(last().query.skillId).toEqual(["skill_9"]);
  });
});
