import type { HttpClient, RequestOptions } from "../http.js";

export type InvestorEventType =
  | "LANDING_VIEW"
  | "DECK_VIEW"
  | "PAGE_VIEW"
  | "CTA_CLICK"
  | "LINK_CLICK"
  | "SCROLL_DEPTH"
  | "TIME_SPENT"
  | "CUSTOM";

export interface TrackInvestorEventBody {
  eventType: InvestorEventType;
  /** The specific page/section viewed (e.g. "hero", "product", "ask"). */
  page?: string;
  metadata?: Record<string, unknown>;
  /** Client-generated session identifier for grouping events. */
  sessionId?: string;
}

/**
 * Public investor deck and landing page access plus engagement event tracking.
 */
export class InvestorsApi {
  constructor(private readonly http: HttpClient) {}

  /** Get an investor deck by slug. */
  getInvestorPage<T = unknown>(slug: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/investors/${encodeURIComponent(slug)}`, options);
  }

  /** Track an investor page event. */
  trackInvestorEvent<T = unknown>(
    slug: string,
    body: TrackInvestorEventBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/investors/${encodeURIComponent(slug)}/events`,
      body,
      options,
    );
  }
}
