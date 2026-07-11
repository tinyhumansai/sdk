import { describe, expect, it } from "vitest";

import { InferenceApi } from "../src/api/inference.js";
import { mockClient } from "./helpers.js";

describe("InferenceApi", () => {
  it("lists models", async () => {
    const { http, last } = mockClient({ body: { object: "list", data: [] } });
    const api = new InferenceApi(http);

    const result = await api.listModels({ with_display: "1" });

    const call = last();
    expect(call.method).toBe("GET");
    expect(call.path).toBe("/openai/v1/models");
    expect(call.query.with_display).toEqual(["1"]);
    // Non-enveloped body is returned as-is.
    expect(result).toEqual({ object: "list", data: [] });
  });

  it("creates a chat completion without unwrapping the envelope", async () => {
    // A body that looks like an envelope must NOT be unwrapped.
    const raw = { success: true, data: { ignored: true }, id: "chatcmpl_1" };
    const { http, last } = mockClient({ body: raw });
    const api = new InferenceApi(http);

    const body = {
      model: "gpt-x",
      messages: [{ role: "user", content: "hi" }],
    };
    const result = await api.createChatCompletion(body);

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/openai/v1/chat/completions");
    expect(call.body).toEqual(body);
    expect(result).toEqual(raw);
  });

  it("creates a text completion", async () => {
    const { http, last } = mockClient({ body: { id: "cmpl_1" } });
    const api = new InferenceApi(http);

    const body = { model: "gpt-x", prompt: "once upon a time" };
    const result = await api.createCompletion(body);

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/openai/v1/completions");
    expect(call.body).toEqual(body);
    expect(result).toEqual({ id: "cmpl_1" });
  });

  it("creates a transcription", async () => {
    const { http, last } = mockClient({ body: { text: "hello world" } });
    const api = new InferenceApi(http);

    const result = await api.createTranscription({ model: "stt-v1" });

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/openai/v1/audio/transcriptions");
    expect(result).toEqual({ text: "hello world" });
  });

  it("creates speech", async () => {
    const { http, last } = mockClient({ body: { audio: "base64" } });
    const api = new InferenceApi(http);

    const body = { text: "hello", voice_id: "v1" };
    const result = await api.createSpeech(body);

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/openai/v1/audio/speech");
    expect(call.body).toEqual(body);
    expect(result).toEqual({ audio: "base64" });
  });

  it("creates embeddings", async () => {
    const { http, last } = mockClient({ body: { object: "list", data: [] } });
    const api = new InferenceApi(http);

    const body = { model: "embedding-v1", input: "text" };
    const result = await api.createEmbeddings(body);

    const call = last();
    expect(call.method).toBe("POST");
    expect(call.path).toBe("/openai/v1/embeddings");
    expect(call.body).toEqual(body);
    expect(result).toEqual({ object: "list", data: [] });
  });
});
