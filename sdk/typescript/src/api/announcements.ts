import type { HttpClient, RequestOptions } from "../http.js";

export type AnnouncementSeverity = "INFO" | "WARNING" | "CRITICAL";

export interface AnnouncementCta {
  label?: string;
  url?: string;
}

export interface Announcement {
  id?: string;
  title?: string;
  body?: string;
  severity?: AnnouncementSeverity;
  cta?: AnnouncementCta;
  startsAt?: string;
  expiresAt?: string;
  createdAt?: string;
}

/**
 * Active in-app announcements for the signed-in user.
 */
export class AnnouncementsApi {
  constructor(private readonly http: HttpClient) {}

  /** Get the latest active announcement for the signed-in user. */
  getLatestAnnouncements<T = Announcement | null>(
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/announcements/latest", options);
  }
}
