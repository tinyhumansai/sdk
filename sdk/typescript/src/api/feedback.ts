import type { HttpClient, RequestOptions } from "../http.js";

export type FeedbackType = "feature" | "bug";
export type FeedbackVoteValue = 1 | -1 | 0;

export interface CreateFeedbackBody {
  type: FeedbackType;
  title: string;
  body: string;
}

export interface IngestFeedbackBody {
  type: FeedbackType;
  title: string;
  body: string;
  product: "backend" | "opencompany" | "openhuman" | "medulla";
  origin?: string;
  externalRef?: string;
}

export interface ListFeedbackQuery {
  type?: FeedbackType;
  status?: "open" | "planned" | "completed";
  sort?: "hot" | "top" | "new";
  page?: number;
  limit?: number;
}

export interface CommentFeedbackBody {
  body: string;
}

export interface VoteFeedbackBody {
  value: FeedbackVoteValue;
}

/**
 * Public feedback board: submit and browse feature requests and bug reports,
 * read individual items with comments, vote, and comment.
 */
export class FeedbackApi {
  constructor(private readonly http: HttpClient) {}

  /** Submit feedback or a bug report (LLM-moderated, rate-limited). */
  createFeedback<T = unknown>(body: CreateFeedbackBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/feedback", body, options);
  }

  /** Ingest authenticated product feedback into the moderation pipeline. */
  ingestFeedback<T = unknown>(
    body: IngestFeedbackBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/feedback/ingest", body, options);
  }

  /** List feedback on the public board. */
  listFeedback<T = unknown>(query?: ListFeedbackQuery, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/feedback", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get a feedback item with its comments. */
  getFeedback<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/feedback/${encodeURIComponent(id)}`, options);
  }

  /** Comment on a feedback item. */
  commentFeedback<T = unknown>(
    id: string,
    body: CommentFeedbackBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/feedback/${encodeURIComponent(id)}/comments`,
      body,
      options,
    );
  }

  /** Up/down-vote a feedback item (value 0 retracts). */
  voteFeedback<T = unknown>(
    id: string,
    body: VoteFeedbackBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(`/feedback/${encodeURIComponent(id)}/vote`, body, options);
  }
}
