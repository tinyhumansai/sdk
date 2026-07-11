import type { HttpClient, RequestOptions } from "../http.js";

/**
 * Service health check.
 */
export class HealthApi {
  constructor(private readonly http: HttpClient) {}

  /** Health check endpoint. Returns server uptime and status information. */
  check<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/", options);
  }
}
