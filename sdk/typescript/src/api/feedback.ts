import type { HttpClient, RequestOptions } from "../http.js";

export type FeedbackType = "feature" | "bug";
export type FeedbackStatus = "open" | "planned" | "completed" | "closed";
export type FeedbackVoteValue = 1 | -1 | 0;

export interface CreateFeedbackBody {
  type: FeedbackType;
  title: string;
  body: string;
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

export interface UpdateFeedbackStatusBody {
  status: FeedbackStatus;
}

export interface VoteFeedbackBody {
  value: FeedbackVoteValue;
}

/**
 * Public feedback board: submit and browse feature requests and bug reports,
 * read individual items with comments, vote, comment, and (admin) update status.
 */
export class FeedbackApi {
  constructor(private readonly http: HttpClient) {}

  /** Submit feedback or a bug report (LLM-moderated, rate-limited). */
  createFeedback<T = unknown>(body: CreateFeedbackBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/feedback", body, options);
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

  /** Update a feedback item's status (admin only). */
  updateFeedbackStatus<T = unknown>(
    id: string,
    body: UpdateFeedbackStatusBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/feedback/${encodeURIComponent(id)}/status`,
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
