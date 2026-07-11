import { describe, expect, it } from "vitest";

import { TeamsApi } from "../src/api/teams.js";
import { mockClient } from "./helpers.js";

describe("TeamsApi", () => {
  it("lists teams and unwraps the envelope", async () => {
    const { http, last } = mockClient({ data: [{ role: "admin" }] });
    const api = new TeamsApi(http);

    const result = await api.listTeams();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams");
    expect(result).toEqual([{ role: "admin" }]);
  });

  it("joins a team with a JSON body", async () => {
    const { http, last } = mockClient({ data: { team: "t_1" } });
    const api = new TeamsApi(http);

    const result = await api.joinTeam({ code: "INV123" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/join");
    expect(call.body).toEqual({ code: "INV123" });
    expect(result).toEqual({ team: "t_1" });
  });

  it("gets the current user's usage insights", async () => {
    const { http, last } = mockClient({ data: { remainingUsd: 5 } });
    const api = new TeamsApi(http);

    await api.getMyUsage();

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams/me/usage");
  });

  it("gets a single team with an encoded path param", async () => {
    const { http, last } = mockClient({ data: { id: "a/b" } });
    const api = new TeamsApi(http);

    await api.getTeam("a/b");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams/a%2Fb");
  });

  it("updates a team with a JSON body", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.updateTeam("t_1", { name: "New Name" });

    const call = last();
    expect(call.method).toBe("PUT");
    expect(call.path).toBe("/teams/t_1");
    expect(call.body).toEqual({ name: "New Name" });
  });

  it("gets the team billing plan", async () => {
    const { http, last } = mockClient({ data: { plan: "PRO" } });
    const api = new TeamsApi(http);

    await api.getBillingPlan("t_1");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams/t_1/billing/plan");
  });

  it("creates a billing portal session", async () => {
    const { http, last } = mockClient({ data: { url: "https://portal" } });
    const api = new TeamsApi(http);

    await api.createBillingPortal("t_1", { returnUrl: "https://app" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/billing/portal");
    expect(call.body).toEqual({ returnUrl: "https://app" });
  });

  it("purchases a plan", async () => {
    const { http, last } = mockClient({ data: {} });
    const api = new TeamsApi(http);

    await api.purchasePlan("t_1", { plan: "PRO_MONTHLY" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/billing/purchase");
    expect(call.body).toEqual({ plan: "PRO_MONTHLY" });
  });

  it("creates an invite", async () => {
    const { http, last } = mockClient({ data: { code: "INV" } });
    const api = new TeamsApi(http);

    await api.createInvite("t_1", { maxUses: 3, expiresInDays: 7 });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/invites");
    expect(call.body).toEqual({ maxUses: 3, expiresInDays: 7 });
  });

  it("lists invites", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new TeamsApi(http);

    await api.listInvites("t_1");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams/t_1/invites");
  });

  it("sends an email invite", async () => {
    const { http, last } = mockClient({ data: { code: "INV" } });
    const api = new TeamsApi(http);

    await api.sendEmailInvite("t_1", { email: "a@b.com" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/invites/email");
    expect(call.body).toEqual({ email: "a@b.com" });
  });

  it("revokes an invite with both path params encoded", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.revokeInvite("t 1", "i/2");

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/teams/t%201/invites/i%2F2");
  });

  it("leaves a team", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.leaveTeam("t_1");

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/leave");
  });

  it("lists members", async () => {
    const { http, last } = mockClient({ data: [] });
    const api = new TeamsApi(http);

    await api.listMembers("t_1");

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/teams/t_1/members");
  });

  it("removes a member with both path params encoded", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.removeMember("t_1", "u/9");

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/teams/t_1/members/u%2F9");
  });

  it("updates a member's role with a JSON body", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.updateMemberRole("t_1", "u_1", { role: "admin" });

    const call = last();
    expect(call.method).toBe("PUT");
    expect(call.path).toBe("/teams/t_1/members/u_1/role");
    expect(call.body).toEqual({ role: "admin" });
  });

  it("switches the active team", async () => {
    const { http, last } = mockClient({ data: null });
    const api = new TeamsApi(http);

    await api.switchTeam("t_1");

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/teams/t_1/switch");
  });
});
