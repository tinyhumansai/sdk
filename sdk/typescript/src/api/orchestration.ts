import type { HttpClient, RequestOptions } from "../http.js";
import type { WorkspaceProfile } from "./medulla.js";

export interface OrchestrationEvent {
  seq: number;
  role: "user" | "assistant" | "system";
  sender: string;
  body: string;
  ts: number;
  kind: string;
}

export interface IngestOrchestrationEventBody {
  protocol: number;
  counterpartAgentId: string;
  sessionId: string;
  event: OrchestrationEvent;
}

export interface OrchestrationWorldDiffQuery {
  session: string;
}

export interface OrchestrationMessagesQuery {
  after?: number;
}

export interface OrchestrationTool {
  name: string;
  description: string;
  parameters?: Record<string, unknown>;
}

export interface RunOrchestrationBody {
  input: string;
  sessionId?: string;
  flavor?: "tui" | "openhuman";
  tools?: OrchestrationTool[];
  options?: {
    workspaceProfiles?: WorkspaceProfile[];
    [key: string]: unknown;
  };
}

export interface OrchestrationToolResult {
  id: string;
  ok: boolean;
  result?: unknown;
  error?: string;
}

export interface ContinueOrchestrationRunBody {
  cycleId: string;
  toolResults?: OrchestrationToolResult[];
}

export class OrchestrationApi {
  constructor(private readonly http: HttpClient) {}

  ingestEvent<T = unknown>(
    body: IngestOrchestrationEventBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/orchestration/v1/events", body, options);
  }

  postWorldDiff<T = unknown>(body?: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/orchestration/v1/world-diff", body, options);
  }

  getWorldDiff<T = unknown>(
    query: OrchestrationWorldDiffQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/orchestration/v1/world-diff", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  listSessions<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/orchestration/v1/sessions", options);
  }

  listSessionMessages<T = unknown>(
    id: string,
    query?: OrchestrationMessagesQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/orchestration/v1/sessions/${encodeURIComponent(id)}/messages`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  getSessionState<T = unknown>(
    id: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/orchestration/v1/sessions/${encodeURIComponent(id)}/state`,
      options,
    );
  }

  run<T = unknown>(
    body: RunOrchestrationBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/orchestration/v1/run", body, options);
  }

  continueRun<T = unknown>(
    body: ContinueOrchestrationRunBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/orchestration/v1/run/continue", body, options);
  }
}
