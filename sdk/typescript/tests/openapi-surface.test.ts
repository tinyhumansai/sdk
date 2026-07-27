import { describe, expect, it } from "vitest";

import { COMMANDS } from "../src/cli-commands.js";
import { TinyHumansClient } from "../src/client.js";

const ADDED_ROUTES = `
DELETE /agent-integrations/file-storage/files/:fileId
DELETE /api-keys/:keyId
DELETE /budgets/seats/:seatId
DELETE /medulla/v1/sessions/:id
DELETE /medulla/v1/tasks/sources/:id
DELETE /medulla/v1/tasks/:id
DELETE /opencompany/instances/:slug
DELETE /opencompany/instances/:slug/custom-domain
GET /agent-integrations/file-storage/files
GET /agent-integrations/file-storage/files/:fileId
GET /agent-integrations/file-storage/files/:fileId/download
GET /agent-integrations/file-storage/public/:fileId
GET /agent-integrations/file-storage/usage
GET /agent-integrations/history-rewards/status
GET /api-keys
GET /budgets
GET /medulla/v1/roster
GET /medulla/v1/routing/strategy
GET /medulla/v1/sessions
GET /medulla/v1/sessions/:id
GET /medulla/v1/sessions/:id/events
GET /medulla/v1/sessions/:id/messages
GET /medulla/v1/sessions/:id/stream
GET /medulla/v1/tasks
GET /medulla/v1/tasks/sources
GET /opencompany/instances
GET /opencompany/instances/:slug
GET /orchestration/v1/sessions
GET /orchestration/v1/sessions/:id/messages
GET /orchestration/v1/sessions/:id/state
GET /orchestration/v1/world-diff
PATCH /agent-integrations/file-storage/files/:fileId
PATCH /budgets/seats/:seatId
PATCH /medulla/v1/tasks/:id
POST /agent-integrations/file-storage/files
POST /agent-integrations/file-storage/files/:fileId/link
POST /agent-integrations/history-rewards/claim
POST /agent-integrations/history-rewards/uploads
POST /api-keys
POST /budgets/seats
POST /feedback/ingest
POST /medulla/v1/sessions
POST /medulla/v1/sessions/:id/abort
POST /medulla/v1/sessions/:id/messages
POST /medulla/v1/tasks
POST /medulla/v1/tasks/sources
POST /medulla/v1/tasks/sources/:id/sync
POST /opencompany/instances
POST /opencompany/instances/:slug/custom-domain/verify
POST /opencompany/instances/:slug/resume
POST /opencompany/instances/:slug/suspend
POST /orchestration/v1/events
POST /orchestration/v1/run
POST /orchestration/v1/run/continue
POST /orchestration/v1/world-diff
POST /rewards/claim
PUT /medulla/v1/routing/strategy
PUT /opencompany/instances/:slug/custom-domain
`
  .trim()
  .split("\n");

describe("non-admin OpenAPI surface", () => {
  it("exposes every requested new route", () => {
    const routes = new Set(COMMANDS.map(({ verb, path }) => `${verb} ${path}`));
    for (const route of ADDED_ROUTES) expect(routes.has(route), route).toBe(true);
  });

  it("does not expose stale or admin-only methods", () => {
    const client = new TinyHumansClient({ baseUrl: "https://api.tinyhumans.ai" });
    const publicClient = client as unknown as Record<string, Record<string, unknown>>;

    expect(publicClient.investors).toBeUndefined();
    expect(publicClient.agentIntegrations!.refreshComposioToolkits).toBeUndefined();
    expect(publicClient.coupons!.createCoupon).toBeUndefined();
    expect(publicClient.feedback!.updateFeedbackStatus).toBeUndefined();
    expect(publicClient.invite!.createCampaignInvite).toBeUndefined();
    expect(publicClient.teams!.updateTeam).toBeUndefined();
    expect(publicClient.teams!.removeMember).toBeUndefined();
    expect(publicClient.teams!.updateMemberRole).toBeUndefined();
  });
});
