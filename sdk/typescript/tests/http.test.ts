import { afterEach, describe, expect, it } from "vitest";

import { HttpClient, TinyHumansError } from "../src/http.js";
import { mockClient } from "./helpers.js";

describe("HttpClient.request", () => {
  it("builds query strings, skipping null/undefined and expanding arrays", async () => {
    const { http, last } = mockClient({ data: { ok: true } });

    await http.get("/things", {
      query: { tags: ["a", "b"], missing: null, absent: undefined, page: 2, flag: false },
    });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/things");
    expect(call.query.tags).toEqual(["a", "b"]);
    expect(call.query.page).toEqual(["2"]);
    expect(call.query.flag).toEqual(["false"]);
    expect(call.query.missing).toBeUndefined();
    expect(call.query.absent).toBeUndefined();
  });

  it("normalizes a path without a leading slash", async () => {
    const { http, last } = mockClient({ data: {} });
    await http.get("things");
    expect(last().path).toBe("/things");
  });

  it("serializes a plain object body as JSON with a content-type header", async () => {
    const { http, last } = mockClient({ data: { id: 1 } });

    await http.post("/things", { name: "x" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.body).toEqual({ name: "x" });
    expect(call.headers["content-type"]).toBe("application/json");
  });

  it("passes a string body through untouched", async () => {
    const { http, last } = mockClient({ data: {} });

    await http.post("/things", "raw-string-body");

    // mockClient tries JSON.parse; a non-JSON string stays a string.
    expect(last().body).toBe("raw-string-body");
  });

  it("passes FormData and Blob bodies through without JSON stringification", async () => {
    const calls: RequestInit[] = [];
    const fetchImpl = (async (_url: string | URL, init?: RequestInit) => {
      calls.push(init!);
      return new Response(JSON.stringify({ success: true, data: {} }), {
        status: 200,
        headers: { "content-type": "application/json" },
      });
    }) as typeof globalThis.fetch;
    const http = new HttpClient({ baseUrl: "https://api.tinyhumans.ai", fetch: fetchImpl });

    const form = new FormData();
    form.append("a", "1");
    await http.post("/upload", form);
    expect(calls[0]!.body).toBeInstanceOf(FormData);

    const blob = new Blob(["hi"]);
    await http.post("/upload", blob);
    expect(calls[1]!.body).toBeInstanceOf(Blob);
  });

  it("returns text when responseType is text", async () => {
    const { http } = mockClient({ text: "plain text" });
    const result = await http.get("/text", { responseType: "text" });
    expect(result).toBe("plain text");
  });

  it("returns the raw Response when responseType is raw", async () => {
    const { http } = mockClient({ data: { ok: true } });
    const result = await http.get<Response>("/raw", { responseType: "raw" });
    expect(result).toBeInstanceOf(Response);
    expect(result.ok).toBe(true);
  });

  it("throws TinyHumansError on a non-ok raw response", async () => {
    const { http } = mockClient({ status: 500, body: { error: "boom" } });
    await expect(http.get("/raw", { responseType: "raw" })).rejects.toBeInstanceOf(TinyHumansError);
  });

  it("returns undefined for a 204 response", async () => {
    const { http } = mockClient({ status: 204 });
    const result = await http.delete("/things/1");
    expect(result).toBeUndefined();
  });

  it("unwraps the success envelope by default", async () => {
    const { http } = mockClient({ data: { id: "x" } });
    const result = await http.get("/thing");
    expect(result).toEqual({ id: "x" });
  });

  it("returns the full body when unwrapEnvelope is false at request level", async () => {
    const { http } = mockClient({ data: { id: "x" } });
    const result = await http.get("/thing", { unwrapEnvelope: false });
    expect(result).toEqual({ success: true, data: { id: "x" } });
  });

  it("returns the full body when the client disables unwrapping", async () => {
    const { http } = mockClient({ data: { id: "x" } }, { unwrapEnvelope: false });
    const result = await http.get("/thing");
    expect(result).toEqual({ success: true, data: { id: "x" } });
  });

  it("returns the body unchanged when it is not a success envelope", async () => {
    const { http } = mockClient({ body: { foo: "bar" } });
    const result = await http.get("/thing");
    expect(result).toEqual({ foo: "bar" });
  });

  it("throws TinyHumansError with parsed body and error message on a non-ok response", async () => {
    const { http } = mockClient({ status: 400, body: { error: "bad input", errorCode: "E1" } });

    let caught: TinyHumansError | undefined;
    try {
      await http.get("/thing");
    } catch (error) {
      caught = error as TinyHumansError;
    }

    expect(caught).toBeInstanceOf(TinyHumansError);
    expect(caught!.status).toBe(400);
    expect(caught!.body).toEqual({ error: "bad input", errorCode: "E1" });
    expect(caught!.message).toBe("bad input");
    expect(caught!.headers).toBeInstanceOf(Headers);
    expect(caught!.toJSON()).toEqual({
      name: "TinyHumansError",
      message: "bad input",
      status: 400,
      body: { error: "bad input", errorCode: "E1" },
    });
  });

  it("uses a default message when the error body has no error field", async () => {
    const { http } = mockClient({ status: 500, body: { other: true } });
    await expect(http.get("/thing")).rejects.toThrow("TinyHumans request failed with HTTP 500");
  });

  it("sets user auth headers and custom headers", async () => {
    const { http, last } = mockClient(
      { data: {} },
      { token: "tok", apiKey: "key" },
    );

    await http.get("/thing", { headers: { "x-custom": "c" } });

    const call = last();
    expect(call.headers["authorization"]).toBe("Bearer tok");
    expect(call.headers["x-api-key"]).toBe("key");
    expect(call.headers["x-admin-service-token"]).toBeUndefined();
    expect(call.headers["x-custom"]).toBe("c");
    expect(call.headers["x-sdk-client"]).toBe("@tinyhumansai/sdk");
  });

  it("strips trailing slashes from the base URL", async () => {
    const { http, last } = mockClient({ data: {} }, { baseUrl: "https://api.tinyhumans.ai///" });
    await http.get("/thing");
    expect(last().url).toBe("https://api.tinyhumans.ai/thing");
  });
});

describe("HttpClient construction", () => {
  const originalFetch = globalThis.fetch;
  afterEach(() => {
    globalThis.fetch = originalFetch;
  });

  it("throws when no fetch implementation is available", () => {
    // Remove the global so the constructor guard fires.
    (globalThis as { fetch?: typeof fetch }).fetch = undefined;
    expect(() => new HttpClient({ baseUrl: "https://api.tinyhumans.ai", fetch: undefined })).toThrow(
      "TinyHumansClient requires a fetch implementation",
    );
  });

  it("falls back to the global fetch when none is provided", () => {
    globalThis.fetch = originalFetch;
    expect(() => new HttpClient({ baseUrl: "https://api.tinyhumans.ai" })).not.toThrow();
  });
});
