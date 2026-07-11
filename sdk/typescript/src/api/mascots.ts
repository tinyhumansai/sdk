import type { HttpClient, RequestOptions } from "../http.js";

export type MeetingPlatform = "gmeet" | "zoom" | "teams" | "webex";

export interface JoinMeetingRiveColors {
  /** Hex color (e.g. "#FF6B35") for the mascot's primary color. */
  primaryColor?: string;
  /** Hex color for the mascot's secondary color. */
  secondaryColor?: string;
}

export interface JoinMeetingBody {
  platform?: MeetingPlatform;
  meetUrl: string;
  /** Name shown in the meeting participant list. */
  displayName?: string;
  /** Name the AI uses in conversation (defaults to displayName or "OpenHuman"). */
  agentName?: string;
  /** Custom system prompt for the meeting LLM. Use {{AGENT_NAME}} as a placeholder. */
  systemPrompt?: string;
  mascotId?: string;
  /** If true, the bot joins silently (no TTS or welcome prompt). */
  muted?: boolean;
  riveColors?: JoinMeetingRiveColors;
}

export interface ListMeetingsQuery {
  limit?: number;
}

/**
 * Mascots: the server-side mascot library, manifests, Rive assets, the
 * interactive demo page, and dispatching the mascot bot into live meetings.
 */
export class MascotsApi {
  constructor(private readonly http: HttpClient) {}

  /** List available mascots. */
  listMascots<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/mascots", options);
  }

  /** Interactive mascot library demo page. */
  getDemo<T = string>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/mascots/demo", options);
  }

  /** Send the mascot bot into a live meeting. */
  joinMeeting<T = unknown>(body: JoinMeetingBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/mascots/join-meeting", body, options);
  }

  /** List the authenticated user's mascot-bot meeting history. */
  listMeetings<T = unknown>(query?: ListMeetingsQuery, options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/mascots/meetings", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get a mascot manifest. */
  getMascot<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/mascots/${encodeURIComponent(id)}`, options);
  }

  /** Download the Rive animation file for a mascot. */
  getMascotRiv<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/mascots/${encodeURIComponent(id)}/riv`, options);
  }
}
