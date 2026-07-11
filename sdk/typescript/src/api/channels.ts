import type { HttpClient, RequestOptions } from "../http.js";

import type { ChannelKind } from "./auth.js";

export interface ChannelMessageButton {
  label?: string;
  url?: string;
  callbackData?: string;
}

export interface SendMessageBody {
  text?: string;
  parseMode?: "markdown" | "plain";
  photoUrl?: string;
  stickerFileId?: string;
  animationUrl?: string;
  channelId?: number | string;
  buttons?: ChannelMessageButton[];
  replyToMessageId?: number | string;
  threadId?: string;
}

export interface AddReactionBody {
  messageId: number | string;
  emoji: string;
  chatId?: number | string;
}

export interface CreateThreadBody {
  title: string;
}

export interface ListThreadsQuery {
  active?: boolean;
}

export interface UpdateThreadBody {
  action: "close" | "reopen";
}

export interface SendTypingBody {
  chatId?: number | string;
  threadId?: string;
}

/**
 * Telegram and Discord channel integration: send and delete rich messages,
 * react, manage conversation threads, and broadcast typing indicators on a
 * user's linked channel.
 */
export class ChannelsApi {
  constructor(private readonly http: HttpClient) {}

  /** Send a rich message to a user's linked channel. */
  sendMessage<T = unknown>(
    channel: ChannelKind,
    body: SendMessageBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/channels/${encodeURIComponent(channel)}/messages`,
      body,
      options,
    );
  }

  /** Delete a message from a user's linked channel. */
  deleteMessage<T = unknown>(
    channel: ChannelKind,
    messageId: number | string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/channels/${encodeURIComponent(channel)}/messages/${encodeURIComponent(messageId)}`,
      options,
    );
  }

  /** React to a message on a user's linked channel. */
  addReaction<T = unknown>(
    channel: ChannelKind,
    body: AddReactionBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/channels/${encodeURIComponent(channel)}/reactions`,
      body,
      options,
    );
  }

  /** Create a new conversation thread. */
  createThread<T = unknown>(
    channel: ChannelKind,
    body: CreateThreadBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/channels/${encodeURIComponent(channel)}/threads`,
      body,
      options,
    );
  }

  /** List conversation threads. */
  listThreads<T = unknown>(
    channel: ChannelKind,
    query?: ListThreadsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(`/channels/${encodeURIComponent(channel)}/threads`, {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Update a thread's status (close or reopen). */
  updateThread<T = unknown>(
    channel: ChannelKind,
    threadId: string,
    body: UpdateThreadBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/channels/${encodeURIComponent(channel)}/threads/${encodeURIComponent(threadId)}`,
      body,
      options,
    );
  }

  /** Broadcast a typing indicator on a user's linked channel. */
  sendTyping<T = unknown>(
    channel: ChannelKind,
    body?: SendTypingBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/channels/${encodeURIComponent(channel)}/typing`,
      body,
      options,
    );
  }
}
