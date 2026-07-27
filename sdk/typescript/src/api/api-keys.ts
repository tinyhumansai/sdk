import type { HttpClient, RequestOptions } from "../http.js";

export type ApiKeyScope = "read" | "write" | "inference";

export interface CreateApiKeyBody {
  name: string;
  scopes?: ApiKeyScope[];
  allowedIps?: string[];
  expiresAt?: string;
}

export class ApiKeysApi {
  constructor(private readonly http: HttpClient) {}

  listApiKeys<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/api-keys", options);
  }

  createApiKey<T = unknown>(
    body: CreateApiKeyBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/api-keys", body, options);
  }

  deleteApiKey<T = unknown>(keyId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/api-keys/${encodeURIComponent(keyId)}`, options);
  }
}
