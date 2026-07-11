import { HttpClient, type TinyHumansClientOptions } from "../src/http.js";

export interface RecordedRequest {
  method: string;
  url: string;
  path: string;
  query: Record<string, string[]>;
  headers: Record<string, string>;
  body: unknown;
}

export interface MockResult {
  http: HttpClient;
  calls: RecordedRequest[];
  last(): RecordedRequest;
}

export interface MockResponseSpec {
  status?: number;
  /** Value returned in the `data` field of a success envelope. */
  data?: unknown;
  /** Full raw body; overrides `data`/envelope when provided. */
  body?: unknown;
  /** When true, `body` is returned as a text response (not JSON). */
  text?: string;
  headers?: Record<string, string>;
}

/**
 * Build an HttpClient whose fetch is replaced with a recorder that returns the
 * provided responses in order (or repeats the last one). Every request is
 * captured in `calls` for assertions.
 */
export function mockClient(
  responses: MockResponseSpec | MockResponseSpec[] = {},
  options: Partial<TinyHumansClientOptions> = {},
): MockResult {
  const queue = Array.isArray(responses) ? [...responses] : [responses];
  const calls: RecordedRequest[] = [];

  const fetchImpl = (async (input: string | URL, init?: RequestInit) => {
    const url = new URL(typeof input === "string" ? input : input.toString());
    const query: Record<string, string[]> = {};
    for (const key of new Set(url.searchParams.keys())) {
      query[key] = url.searchParams.getAll(key);
    }
    const headers: Record<string, string> = {};
    new Headers(init?.headers).forEach((value, key) => {
      headers[key] = value;
    });
    let body: unknown;
    if (typeof init?.body === "string") {
      try {
        body = JSON.parse(init.body);
      } catch {
        body = init.body;
      }
    } else if (init?.body !== undefined) {
      body = init.body;
    }
    calls.push({
      method: (init?.method ?? "GET").toUpperCase(),
      url: url.toString(),
      path: url.pathname,
      query,
      headers,
      body,
    });

    const spec = queue.length > 1 ? queue.shift()! : (queue[0] ?? {});
    const status = spec.status ?? 200;
    const responseHeaders = new Headers({
      "content-type": spec.text !== undefined ? "text/plain" : "application/json",
      ...spec.headers,
    });
    const payload =
      spec.text !== undefined
        ? spec.text
        : JSON.stringify(
            spec.body !== undefined ? spec.body : { success: true, data: spec.data ?? null },
          );
    return new Response(status === 204 ? null : payload, {
      status,
      headers: responseHeaders,
    });
  }) as typeof globalThis.fetch;

  const http = new HttpClient({
    baseUrl: options.baseUrl ?? "https://api.tinyhumans.ai",
    fetch: fetchImpl,
    ...options,
  });

  return {
    http,
    calls,
    last: () => calls[calls.length - 1]!,
  };
}
