import { describe, expect, it } from "vitest";

import { InviteApi } from "../src/api/invite.js";
import { mockClient } from "./helpers.js";

describe("InviteApi", () => {
  it("creates a campaign invite with a JSON body", async () => {
    const { http, last } = mockClient({ status: 201, data: { code: "SUMMER" } });
    const api = new InviteApi(http);

    const result = await api.createCampaignInvite({ maxUses: 100, code: "SUMMER" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/invite/campaign");
    expect(call.body).toEqual({ maxUses: 100, code: "SUMMER" });
    expect(result).toEqual({ code: "SUMMER" });
  });

  it("lists campaign invites", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new InviteApi(http);

    await api.listCampaignInvites();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/invite/campaign");
  });

  it("encodes the codeId when deleting a campaign invite", async () => {
    const { http, last } = mockClient({ data: { deactivated: true } });
    const api = new InviteApi(http);

    await api.deleteCampaignInvite("code id/1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/invite/campaign/code%20id%2F1");
  });

  it("lists the current user's codes", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new InviteApi(http);

    await api.listMyCodes();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/invite/my-codes");
  });

  it("redeems an invite with a JSON body", async () => {
    const { http, last } = mockClient({ data: { redeemed: true } });
    const api = new InviteApi(http);

    await api.redeemInvite({ code: "ABCD" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/invite/redeem");
    expect(call.body).toEqual({ code: "ABCD" });
  });

  it("checks invite status with the code as a query parameter", async () => {
    const { http, last } = mockClient({ data: { valid: true } });
    const api = new InviteApi(http);

    await api.getInviteStatus({ code: "ABCD" });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/invite/status");
    expect(call.query.code).toEqual(["ABCD"]);
  });
});
