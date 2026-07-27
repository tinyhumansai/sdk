import type { HttpClient, RequestOptions } from "../http.js";

export type TeamRole = "admin" | "billing_manager" | "member";

export type TeamPlan = "FREE" | "BASIC" | "PRO";

export type TeamPurchasePlan =
  | "BASIC_MONTHLY"
  | "BASIC_YEARLY"
  | "PRO_MONTHLY"
  | "PRO_YEARLY";

export interface JoinTeamBody {
  /** Team invite code. */
  code: string;
}

export interface CreateBillingPortalBody {
  /** URL to return to after the portal session. */
  returnUrl?: string;
}

export interface PurchasePlanBody {
  plan: TeamPurchasePlan;
  successUrl?: string;
  cancelUrl?: string;
}

export interface CreateInviteBody {
  maxUses?: number;
  expiresInDays?: number;
}

export interface SendEmailInviteBody {
  email: string;
}

/**
 * Team-scoped membership and access: listing teams, usage insights, invites,
 * joining/leaving/switching teams, member management, and billing plan helpers.
 */
export class TeamsApi {
  constructor(private readonly http: HttpClient) {}

  /** List all teams the authenticated user belongs to, with their role in each. */
  listTeams<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/teams", options);
  }

  /** Join a team using an invite code. */
  joinTeam<T = unknown>(body: JoinTeamBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/teams/join", body, options);
  }

  /** Get usage and spend insights plus remaining budget for the current cycle. */
  getMyUsage<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/teams/me/usage", options);
  }

  /** Get details for a single team. */
  getTeam<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/teams/${encodeURIComponent(teamId)}`, options);
  }

  /** Get the team's current subscription plan. */
  getBillingPlan<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/teams/${encodeURIComponent(teamId)}/billing/plan`,
      options,
    );
  }

  /** Create a Stripe billing portal session for the team. */
  createBillingPortal<T = unknown>(
    teamId: string,
    body?: CreateBillingPortalBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/billing/portal`,
      body,
      options,
    );
  }

  /** Purchase a subscription plan for the team (creates a checkout session). */
  purchasePlan<T = unknown>(
    teamId: string,
    body: PurchasePlanBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/billing/purchase`,
      body,
      options,
    );
  }

  /** Create a shareable team invite. */
  createInvite<T = unknown>(
    teamId: string,
    body?: CreateInviteBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/invites`,
      body,
      options,
    );
  }

  /** List a team's invites. */
  listInvites<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/teams/${encodeURIComponent(teamId)}/invites`, options);
  }

  /** Send a team invite via email. */
  sendEmailInvite<T = unknown>(
    teamId: string,
    body: SendEmailInviteBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/invites/email`,
      body,
      options,
    );
  }

  /** Revoke a team invite. */
  revokeInvite<T = unknown>(
    teamId: string,
    inviteId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/teams/${encodeURIComponent(teamId)}/invites/${encodeURIComponent(inviteId)}`,
      options,
    );
  }

  /** Leave a team. */
  leaveTeam<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/leave`,
      undefined,
      options,
    );
  }

  /** List a team's members. */
  listMembers<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/teams/${encodeURIComponent(teamId)}/members`, options);
  }

  /** Switch the user's active team. */
  switchTeam<T = unknown>(teamId: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/teams/${encodeURIComponent(teamId)}/switch`,
      undefined,
      options,
    );
  }
}
