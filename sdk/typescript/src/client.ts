import { HttpClient, type TinyHumansClientOptions } from "./http.js";

import { AdminApi } from "./api/admin.js";
import { AgentIntegrationsApi } from "./api/agent-integrations.js";
import { AnnouncementsApi } from "./api/announcements.js";
import { AuthApi } from "./api/auth.js";
import { ChannelsApi } from "./api/channels.js";
import { CouponsApi } from "./api/coupons.js";
import { FeedbackApi } from "./api/feedback.js";
import { HealthApi } from "./api/health.js";
import { InferenceApi } from "./api/inference.js";
import { InvestorsApi } from "./api/investors.js";
import { InviteApi } from "./api/invite.js";
import { MascotsApi } from "./api/mascots.js";
import { PaymentsApi } from "./api/payments.js";
import { RedirectApi } from "./api/redirect.js";
import { ReferralApi } from "./api/referral.js";
import { RewardsApi } from "./api/rewards.js";
import { TeamsApi } from "./api/teams.js";
import { WebhooksApi } from "./api/webhooks.js";

/**
 * Typed client for the TinyHumans backend. Each property is a namespace client
 * exposing one method per deployed operation. `raw` is the escape hatch for
 * routes not yet surfaced as typed methods.
 */
export class TinyHumansClient {
  readonly raw: HttpClient;

  readonly admin: AdminApi;
  readonly agentIntegrations: AgentIntegrationsApi;
  readonly announcements: AnnouncementsApi;
  readonly auth: AuthApi;
  readonly channels: ChannelsApi;
  readonly coupons: CouponsApi;
  readonly feedback: FeedbackApi;
  readonly health: HealthApi;
  readonly inference: InferenceApi;
  readonly investors: InvestorsApi;
  readonly invite: InviteApi;
  readonly mascots: MascotsApi;
  readonly payments: PaymentsApi;
  readonly redirect: RedirectApi;
  readonly referral: ReferralApi;
  readonly rewards: RewardsApi;
  readonly teams: TeamsApi;
  readonly webhooks: WebhooksApi;

  constructor(options: TinyHumansClientOptions) {
    this.raw = new HttpClient(options);

    this.admin = new AdminApi(this.raw);
    this.agentIntegrations = new AgentIntegrationsApi(this.raw);
    this.announcements = new AnnouncementsApi(this.raw);
    this.auth = new AuthApi(this.raw);
    this.channels = new ChannelsApi(this.raw);
    this.coupons = new CouponsApi(this.raw);
    this.feedback = new FeedbackApi(this.raw);
    this.health = new HealthApi(this.raw);
    this.inference = new InferenceApi(this.raw);
    this.investors = new InvestorsApi(this.raw);
    this.invite = new InviteApi(this.raw);
    this.mascots = new MascotsApi(this.raw);
    this.payments = new PaymentsApi(this.raw);
    this.redirect = new RedirectApi(this.raw);
    this.referral = new ReferralApi(this.raw);
    this.rewards = new RewardsApi(this.raw);
    this.teams = new TeamsApi(this.raw);
    this.webhooks = new WebhooksApi(this.raw);
  }

  /** Fetch the deployed OpenAPI document (not enveloped). */
  swagger<T = unknown>(): Promise<T> {
    return this.raw.get<T>("/swagger.json", { unwrapEnvelope: false });
  }
}
