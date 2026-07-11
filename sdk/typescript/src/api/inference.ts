import type { HttpClient, RequestOptions } from "../http.js";

/** A single chat message. Content is permissive to mirror OpenAI's schema. */
export interface ChatMessage {
  role: string;
  content: unknown;
}

/** Request body for POST /openai/v1/chat/completions. */
export interface ChatCompletionRequest {
  model: string;
  messages: ChatMessage[];
  [key: string]: unknown;
}

/** Request body for POST /openai/v1/completions. */
export interface CompletionRequest {
  model: string;
  prompt: string;
  [key: string]: unknown;
}

/** Request body for POST /openai/v1/embeddings. */
export interface EmbeddingsRequest {
  model: string;
  input: unknown;
  [key: string]: unknown;
}

/** Request body for POST /openai/v1/audio/speech. */
export interface SpeechRequest {
  text: string;
  [key: string]: unknown;
}

/**
 * Request body for POST /openai/v1/audio/transcriptions.
 *
 * The endpoint expects multipart/form-data. Pass a `FormData` instance directly
 * as the body, or this permissive shape when constructing one yourself.
 */
export interface TranscriptionRequest {
  file?: unknown;
  [key: string]: unknown;
}

/** Query parameters for GET /openai/v1/models. */
export interface ListModelsQuery {
  with_display?: "1" | "true";
}

/**
 * OpenAI-compatible inference: model listing, chat/text completions,
 * embeddings, speech synthesis, and audio transcription.
 *
 * These endpoints return native OpenAI-format bodies and are NOT wrapped in the
 * TinyHumans `{ success, data }` envelope, so every method defaults to
 * `unwrapEnvelope: false`. Callers can override via `options`.
 */
export class InferenceApi {
  constructor(private readonly http: HttpClient) {}

  /** List available models in OpenAI format. */
  listModels<T = unknown>(query?: ListModelsQuery, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/openai/v1/models", {
      unwrapEnvelope: false,
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Create a chat completion. */
  createChatCompletion<T = unknown>(
    body: ChatCompletionRequest,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/openai/v1/chat/completions", body, {
      unwrapEnvelope: false,
      ...options,
    });
  }

  /** Create a text completion. */
  createCompletion<T = unknown>(
    body: CompletionRequest,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/openai/v1/completions", body, {
      unwrapEnvelope: false,
      ...options,
    });
  }

  /** Transcribe audio to text. */
  createTranscription<T = unknown>(
    body: TranscriptionRequest | FormData,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/openai/v1/audio/transcriptions", body, {
      unwrapEnvelope: false,
      ...options,
    });
  }

  /**
   * Synthesize speech from text via ElevenLabs (mp3).
   *
   * Returns audio bytes by default; use `options.responseType` to control how
   * the response is read (e.g. `"raw"` to access the raw `Response`).
   */
  createSpeech<T = unknown>(body: SpeechRequest, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/openai/v1/audio/speech", body, {
      unwrapEnvelope: false,
      ...options,
    });
  }

  /** Create text embeddings via Voyage AI. */
  createEmbeddings<T = unknown>(
    body: EmbeddingsRequest,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/openai/v1/embeddings", body, {
      unwrapEnvelope: false,
      ...options,
    });
  }
}
