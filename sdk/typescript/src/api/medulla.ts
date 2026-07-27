import type { HttpClient, RequestOptions } from "../http.js";

export interface WorkspaceProfile {
  workspace: string;
  medullaMd: string;
}

export interface CreateMedullaSessionBody {
  title?: string;
  meta?: Record<string, unknown>;
  asyncDelegation?: boolean;
  flavor?: "tui" | "openhuman";
  workspaceProfiles?: WorkspaceProfile[];
}

export interface SendMedullaMessageBody {
  body: string;
}

export interface SendMedullaMessageQuery {
  sync?: string;
}

export interface MedullaAfterQuery {
  after?: number;
}

export interface MedullaStreamQuery extends MedullaAfterQuery {
  token?: string;
  lastEventId?: number;
}

export type MedullaRoutingStrategy =
  | "manual"
  | "balanced"
  | "cpuFirst"
  | "memoryFirst";

export interface UpdateMedullaRoutingStrategyBody {
  strategy: MedullaRoutingStrategy;
}

export type MedullaTaskStatus = "open" | "inProgress" | "done" | "cancelled";

export interface CreateMedullaTaskBody {
  title: string;
  description?: string;
  status?: MedullaTaskStatus;
  recurrence?: Record<string, unknown>;
}

export type UpdateMedullaTaskBody = Partial<CreateMedullaTaskBody>;

export interface CreateMedullaTaskSourceBody {
  repository: string;
  state?: "open" | "closed" | "all";
  labels?: string[];
  filter?: string;
  token?: string;
  enabled?: boolean;
}

export class MedullaApi {
  constructor(private readonly http: HttpClient) {}

  createSession<T = unknown>(
    body?: CreateMedullaSessionBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/medulla/v1/sessions", body, options);
  }

  listSessions<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/medulla/v1/sessions", options);
  }

  getSession<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(`/medulla/v1/sessions/${encodeURIComponent(id)}`, options);
  }

  deleteSession<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}`,
      options,
    );
  }

  sendMessage<T = unknown>(
    id: string,
    body: SendMedullaMessageBody,
    query?: SendMedullaMessageQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}/messages`,
      body,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  listMessages<T = unknown>(
    id: string,
    query?: MedullaAfterQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}/messages`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  listEvents<T = unknown>(
    id: string,
    query?: MedullaAfterQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}/events`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  abortSession<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}/abort`,
      undefined,
      options,
    );
  }

  streamSession<T = Response>(
    id: string,
    query?: MedullaStreamQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/medulla/v1/sessions/${encodeURIComponent(id)}/stream`,
      {
        ...options,
        query: { ...query, ...options?.query },
        responseType: options?.responseType ?? "raw",
      },
    );
  }

  getRoster<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/medulla/v1/roster", options);
  }

  getRoutingStrategy<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/medulla/v1/routing/strategy", options);
  }

  updateRoutingStrategy<T = unknown>(
    body: UpdateMedullaRoutingStrategyBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.put<T>("/medulla/v1/routing/strategy", body, options);
  }

  listTasks<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/medulla/v1/tasks", options);
  }

  createTask<T = unknown>(
    body: CreateMedullaTaskBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/medulla/v1/tasks", body, options);
  }

  updateTask<T = unknown>(
    id: string,
    body: UpdateMedullaTaskBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/medulla/v1/tasks/${encodeURIComponent(id)}`,
      body,
      options,
    );
  }

  deleteTask<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(`/medulla/v1/tasks/${encodeURIComponent(id)}`, options);
  }

  listTaskSources<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/medulla/v1/tasks/sources", options);
  }

  createTaskSource<T = unknown>(
    body: CreateMedullaTaskSourceBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/medulla/v1/tasks/sources", body, options);
  }

  syncTaskSource<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/medulla/v1/tasks/sources/${encodeURIComponent(id)}/sync`,
      undefined,
      options,
    );
  }

  deleteTaskSource<T = unknown>(id: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(
      `/medulla/v1/tasks/sources/${encodeURIComponent(id)}`,
      options,
    );
  }
}
