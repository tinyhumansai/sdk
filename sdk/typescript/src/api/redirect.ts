import type { HttpClient, RequestOptions } from "../http.js";

/**
 * Short-link resolution.
 */
export class RedirectApi {
  constructor(private readonly http: HttpClient) {}

  /** Redirect a short link code to its original URL (returns a 302 redirect). */
  resolveRedirect<T = unknown>(code: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/r/${encodeURIComponent(code)}`, options);
  }
}
