import { describe, expect, it } from "vitest";

import { AdminApi } from "../src/api/admin.js";
import { mockClient } from "./helpers.js";

describe("AdminApi", () => {
  // Analytics

  it("gets activity analytics with a date range query", async () => {
    const { http, last } = mockClient({ data: { events: [] } });
    const api = new AdminApi(http);

    const result = await api.getAnalyticsActivity({
      startDate: "2026-01-01",
      endDate: "2026-02-01",
    });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/admin/analytics/activity");
    expect(call.query.startDate).toEqual(["2026-01-01"]);
    expect(call.query.endDate).toEqual(["2026-02-01"]);
    expect(result).toEqual({ events: [] });
  });

  it("gets channel analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsChannels({ granularity: "day", topLimit: 5 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/admin/analytics/channels");
    expect(call.query.granularity).toEqual(["day"]);
    expect(call.query.topLimit).toEqual(["5"]);
  });

  it("gets the dashboard summary", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsDashboard({ engagementThreshold: 3 });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/admin/analytics/dashboard");
    expect(call.query.engagementThreshold).toEqual(["3"]);
  });

  it("gets backend event analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsBackendEvents();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/events/backend");
  });

  it("gets financial details", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsFinancialsDetails();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/financials/details");
  });

  it("gets home metrics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsHome({ granularity: "week" });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/home");
    expect(last().query.granularity).toEqual(["week"]);
  });

  it("gets inference usage analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsInferenceUsage({ topLimit: 10 });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/inference/usage");
    expect(last().query.topLimit).toEqual(["10"]);
  });

  it("gets the leaderboard", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsLeaderboard({ sortBy: "prompts", excludeTeamEmails: true });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/leaderboard");
    expect(last().query.sortBy).toEqual(["prompts"]);
    expect(last().query.excludeTeamEmails).toEqual(["true"]);
  });

  it("gets provider credits", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnalyticsProviderCredits({ category: "llm" });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/analytics/providers/credits");
    expect(last().query.category).toEqual(["llm"]);
  });

  // Announcements

  it("lists announcements", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.listAnnouncements({ isActive: "true", page: 2 });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/announcements");
    expect(last().query.isActive).toEqual(["true"]);
    expect(last().query.page).toEqual(["2"]);
  });

  it("creates an announcement with a JSON body", async () => {
    const { http, last } = mockClient({ status: 201, data: { id: "a_1" } });
    const api = new AdminApi(http);

    const result = await api.createAnnouncement({
      title: "Hi",
      body: "Body",
      severity: "WARNING",
    });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/admin/announcements");
    expect(call.body).toEqual({ title: "Hi", body: "Body", severity: "WARNING" });
    expect(result).toEqual({ id: "a_1" });
  });

  it("gets an announcement by id, encoding the path param", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAnnouncement("ann/1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/announcements/ann%2F1");
  });

  it("updates an announcement", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.updateAnnouncement("ann_1", { isActive: false });

    const call = last();
    expect(call.method).toBe("PATCH");
    expect(call.path).toBe("/admin/announcements/ann_1");
    expect(call.body).toEqual({ isActive: false });
  });

  it("deletes an announcement", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.deleteAnnouncement("ann_1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/admin/announcements/ann_1");
  });

  // Management

  it("lists audit logs", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.listAuditLogs({ action: "CREDIT_GRANT", adminId: "adm_1" });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/audit-logs");
    expect(last().query.action).toEqual(["CREDIT_GRANT"]);
    expect(last().query.adminId).toEqual(["adm_1"]);
  });

  // Coupons

  it("creates a coupon", async () => {
    const { http, last } = mockClient({ status: 201, data: { id: "c_1" } });
    const api = new AdminApi(http);

    await api.createCoupon({ amountUsd: 5 });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/coupons");
    expect(last().body).toEqual({ amountUsd: 5 });
  });

  it("bulk-creates coupons", async () => {
    const { http, last } = mockClient({ status: 201 });
    const api = new AdminApi(http);

    await api.bulkCreateCoupons({ count: 10, amountUsd: 2 });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/coupons/bulk");
    expect(last().body).toEqual({ count: 10, amountUsd: 2 });
  });

  it("updates a coupon", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.updateCoupon("cpn_1", { isActive: false });

    expect(last().method).toBe("PATCH");
    expect(last().path).toBe("/admin/coupons/cpn_1");
    expect(last().body).toEqual({ isActive: false });
  });

  it("deletes a coupon", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.deleteCoupon("cpn_1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/admin/coupons/cpn_1");
  });

  // Investors

  it("creates an investor", async () => {
    const { http, last } = mockClient({ status: 201 });
    const api = new AdminApi(http);

    await api.createInvestor({ name: "Acme" });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/investors");
    expect(last().body).toEqual({ name: "Acme" });
  });

  it("gets an investor", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getInvestor("inv_1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/investors/inv_1");
  });

  it("updates an investor with PUT", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.updateInvestor("inv_1", { name: "New" });

    expect(last().method).toBe("PUT");
    expect(last().path).toBe("/admin/investors/inv_1");
    expect(last().body).toEqual({ name: "New" });
  });

  it("deletes an investor", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.deleteInvestor("inv_1");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/admin/investors/inv_1");
  });

  it("gets investor analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getInvestorAnalytics("inv_1", { startDate: "2026-01-01" });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/investors/inv_1/analytics");
    expect(last().query.startDate).toEqual(["2026-01-01"]);
  });

  it("lists investor events", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.listInvestorEvents("inv_1", { limit: 25 });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/investors/inv_1/events");
    expect(last().query.limit).toEqual(["25"]);
  });

  // Mascots

  it("lists mascots", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.listMascots();

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/mascots");
  });

  it("creates a mascot", async () => {
    const { http, last } = mockClient({ status: 201 });
    const api = new AdminApi(http);

    await api.createMascot({
      id: "robo",
      name: "Robo",
      version: "1",
      rivBase64: "AAAA",
    });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/mascots");
    expect(last().body).toEqual({
      id: "robo",
      name: "Robo",
      version: "1",
      rivBase64: "AAAA",
    });
  });

  it("updates a mascot with PUT", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.updateMascot("robo", { version: "2" });

    expect(last().method).toBe("PUT");
    expect(last().path).toBe("/admin/mascots/robo");
    expect(last().body).toEqual({ version: "2" });
  });

  it("deletes a mascot", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.deleteMascot("robo");

    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/admin/mascots/robo");
  });

  // Users

  it("bulk-grants user credits", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.bulkGrantUserCredits({ userIds: ["u_1", "u_2"], credits: 100 });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/users/credits/bulk");
    expect(last().body).toEqual({ userIds: ["u_1", "u_2"], credits: 100 });
  });

  it("gets an admin user", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getAdminUser("u_1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/users/u_1");
  });

  it("gets per-user channel analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getUserChannelAnalytics("u_1", { granularity: "month" });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/users/u_1/analytics/channels");
    expect(last().query.granularity).toEqual(["month"]);
  });

  it("gets per-user usage analytics", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.getUserUsageAnalytics("u_1");

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/users/u_1/analytics/usage");
  });

  it("grants user credits with a JSON body", async () => {
    const { http, last } = mockClient({ data: { balance: 42 } });
    const api = new AdminApi(http);

    const result = await api.grantUserCredits("u_1", {
      action: "ADD",
      amountUsd: 5,
      reason: "goodwill",
    });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/admin/users/u_1/credits");
    expect(call.body).toEqual({ action: "ADD", amountUsd: 5, reason: "goodwill" });
    expect(result).toEqual({ balance: 42 });
  });

  it("lists user credit transactions", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.listUserCreditTransactions("u_1", { page: 1, limit: 20 });

    expect(last().method).toBe("GET");
    expect(last().path).toBe("/admin/users/u_1/credits/transactions");
    expect(last().query.page).toEqual(["1"]);
    expect(last().query.limit).toEqual(["20"]);
  });

  it("grants a subscription", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.grantUserSubscription("u_1", { plan: "pro", reason: "beta" });

    expect(last().method).toBe("POST");
    expect(last().path).toBe("/admin/users/u_1/subscription");
    expect(last().body).toEqual({ plan: "pro", reason: "beta" });
  });

  it("cancels a subscription with a DELETE body", async () => {
    const { http, last } = mockClient();
    const api = new AdminApi(http);

    await api.cancelUserSubscription("u_1", { reason: "refund" });

    const call = last();
    expect(call.method).toBe("DELETE");
    expect(call.path).toBe("/admin/users/u_1/subscription");
    expect(call.body).toEqual({ reason: "refund" });
  });
});
