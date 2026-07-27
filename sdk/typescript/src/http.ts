export type QueryValue = string | number | boolean | null | undefined;
export type Query = Record<string, QueryValue | QueryValue[]>;

export interface TinyHumansClientOptions {
  baseUrl: string;
  token?: string;
  apiKey?: string;
  headers?: Record<string, string>;
  fetch?: typeof globalThis.fetch;
  unwrapEnvelope?: boolean;
}

export interface RequestOptions {
  query?: Query;
  body?: unknown;
  headers?: Record<string, string>;
  responseType?: "json" | "text" | "raw";
  unwrapEnvelope?: boolean;
}

export class TinyHumansError extends Error {
  constructor(
    public readonly status: number,
    public readonly body: unknown,
    public readonly headers: Headers,
    message?: string,
  ) {
    super(message ?? `TinyHumans request failed with HTTP ${status}`);
    this.name = "TinyHumansError";
  }

  toJSON(): Record<string, unknown> {
    return {
      name: this.name,
      message: this.message,
      status: this.status,
      body: this.body,
    };
  }
}

export class HttpClient {
  private readonly baseUrl: string;
  private readonly fetchImpl: typeof globalThis.fetch;
  private readonly unwrapEnvelope: boolean;

  constructor(private readonly options: TinyHumansClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/+$/, "");
    this.fetchImpl = options.fetch ?? globalThis.fetch;
    this.unwrapEnvelope = options.unwrapEnvelope ?? true;

    if (!this.fetchImpl) {
      throw new Error("TinyHumansClient requires a fetch implementation");
    }
  }

  async request<T = unknown>(
    method: string,
    path: string,
    options: RequestOptions = {},
  ): Promise<T> {
    const url = this.buildUrl(path, options.query);
    const headers = this.buildHeaders(options);
    const init: RequestInit = { method, headers };

    if (options.body !== undefined) {
      headers.set("content-type", headers.get("content-type") ?? "application/json");
      init.body =
        typeof options.body === "string" ||
        options.body instanceof FormData ||
        options.body instanceof Blob
          ? (options.body as BodyInit)
          : JSON.stringify(options.body);
    }

    const response = await this.fetchImpl(url, init);
    if (options.responseType === "raw") {
      if (!response.ok) {
        throw new TinyHumansError(response.status, await safeBody(response), response.headers);
      }
      return response as T;
    }

    const body =
      options.responseType === "text" ? await response.text() : await safeBody(response);

    if (!response.ok) {
      const message =
        typeof body === "object" && body && "error" in body
          ? String((body as { error?: unknown }).error)
          : undefined;
      throw new TinyHumansError(response.status, body, response.headers, message);
    }

    if (response.status === 204) return undefined as T;

    const shouldUnwrap = options.unwrapEnvelope ?? this.unwrapEnvelope;
    if (
      shouldUnwrap &&
      body &&
      typeof body === "object" &&
      "success" in body &&
      (body as { success?: unknown }).success === true &&
      "data" in body
    ) {
      return (body as { data: T }).data;
    }

    return body as T;
  }

  get<T = unknown>(path: string, options?: RequestOptions): Promise<T> {
    return this.request<T>("GET", path, options);
  }

  post<T = unknown>(path: string, body?: unknown, options: RequestOptions = {}): Promise<T> {
    return this.request<T>("POST", path, { ...options, body });
  }

  put<T = unknown>(path: string, body?: unknown, options: RequestOptions = {}): Promise<T> {
    return this.request<T>("PUT", path, { ...options, body });
  }

  patch<T = unknown>(path: string, body?: unknown, options: RequestOptions = {}): Promise<T> {
    return this.request<T>("PATCH", path, { ...options, body });
  }

  delete<T = unknown>(path: string, options?: RequestOptions): Promise<T> {
    return this.request<T>("DELETE", path, options);
  }

  private buildUrl(path: string, query?: Query): URL {
    const normalizedPath = path.startsWith("/") ? path : `/${path}`;
    const url = new URL(`${this.baseUrl}${normalizedPath}`);
    for (const [key, value] of Object.entries(query ?? {})) {
      const values = Array.isArray(value) ? value : [value];
      for (const item of values) {
        if (item !== undefined && item !== null) url.searchParams.append(key, String(item));
      }
    }
    return url;
  }

  private buildHeaders(options: RequestOptions): Headers {
    const headers = new Headers({
      "accept": "application/json",
      "x-sdk-client": "@tinyhumansai/sdk",
      ...this.options.headers,
      ...options.headers,
    });
    if (this.options.token) headers.set("authorization", `Bearer ${this.options.token}`);
    if (this.options.apiKey) headers.set("x-api-key", this.options.apiKey);
    return headers;
  }
}

async function safeBody(response: Response): Promise<unknown> {
  const text = await response.text();
  if (!text) return undefined;
  try {
    return JSON.parse(text);
  } catch {
    return text;
  }
}
