import type { HttpClient, RequestOptions } from "../http.js";

export type ChannelKind = "telegram" | "discord";

export interface SendEmailLinkBody {
  email: string;
  frontendRedirectUri?: string;
}

export interface ConsumeLoginTokenBody {
  token: string;
}

export interface CreateIntegrationTokenBody {
  /** Client encryption key (32-byte raw string, base64, or base64url). */
  key: string;
}

export interface OAuthConnectQuery {
  responseType?: string;
  skillId?: string;
}

export interface OAuthLoginQuery {
  redirect?: string;
  redirectUri?: string;
}

export interface CurrentUser {
  id: string;
  email?: string;
  [key: string]: unknown;
}

/**
 * Auth and account state: magic-link email login, OAuth connect/login/callback,
 * the current user, channel link tokens, and third-party integration tokens.
 */
export class AuthApi {
  constructor(private readonly http: HttpClient) {}

  /** Create a short-lived token to link a Telegram or Discord account. */
  createChannelLinkToken<T = unknown>(
    channel: ChannelKind,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/auth/channels/${encodeURIComponent(channel)}/link-token`,
      undefined,
      options,
    );
  }

  /** Send a magic link for email login. */
  sendEmailLink<T = unknown>(body: SendEmailLinkBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/auth/email/send-link", body, options);
  }

  /** Verify a magic link and complete email login (returns a 302 redirect). */
  verifyEmail<T = unknown>(token: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/auth/email/verify", { ...options, query: { token } });
  }

  /** Consume a one-time login token, exchanging it for a session. */
  consumeLoginToken<T = unknown>(
    body: ConsumeLoginTokenBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/auth/login-token/consume", body, options);
  }

  /** Get the currently authenticated user. */
  me<T = CurrentUser>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/auth/me", options);
  }

  /** List the user's connected third-party integrations. */
  listIntegrations<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/auth/integrations", options);
  }

  /** Disconnect a third-party integration. */
  deleteIntegration<T = unknown>(
    integrationId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/auth/integrations/${encodeURIComponent(integrationId)}`,
      options,
    );
  }

  /** Issue an access token for a connected integration. */
  createIntegrationToken<T = unknown>(
    integrationId: string,
    body: CreateIntegrationTokenBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/auth/integrations/${encodeURIComponent(integrationId)}/tokens`,
      body,
      options,
    );
  }

  /** OAuth provider callback endpoint (returns a redirect). */
  oauthCallback<T = unknown>(provider: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/auth/${encodeURIComponent(provider)}/callback`, options);
  }

  /** Start an OAuth connect flow for a provider. */
  oauthConnect<T = unknown>(
    provider: string,
    query?: OAuthConnectQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(`/auth/${encodeURIComponent(provider)}/connect`, {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Start an OAuth login flow for a provider (returns a redirect). */
  oauthLogin<T = unknown>(
    provider: string,
    query?: OAuthLoginQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(`/auth/${encodeURIComponent(provider)}/login`, {
      ...options,
      query: { ...query, ...options?.query },
    });
  }
}
