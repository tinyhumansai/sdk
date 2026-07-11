import { HttpClient, type RequestOptions, type TinyHumansClientOptions } from "./http.js";

export class NamespaceClient {
  constructor(
    private readonly http: HttpClient,
    private readonly basePath: string,
  ) {}

  get<T = unknown>(path = "/", options?: RequestOptions): Promise<T> {
    return this.http.get<T>(this.path(path), options);
  }

  post<T = unknown>(path = "/", body?: unknown, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(this.path(path), body, options);
  }

  put<T = unknown>(path = "/", body?: unknown, options?: RequestOptions): Promise<T> {
    return this.http.put<T>(this.path(path), body, options);
  }

  patch<T = unknown>(path = "/", body?: unknown, options?: RequestOptions): Promise<T> {
    return this.http.patch<T>(this.path(path), body, options);
  }

  delete<T = unknown>(path = "/", options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(this.path(path), options);
  }

  private path(path: string): string {
    const suffix = path === "/" ? "" : path.startsWith("/") ? path : `/${path}`;
    return `${this.basePath}${suffix || ""}` || "/";
  }
}

export class TinyHumansClient {
  readonly raw: HttpClient;
  readonly apiKeys: NamespaceClient;
  readonly auth: NamespaceClient;
  readonly inference: NamespaceClient;
  readonly agentIntegrations: NamespaceClient;
  readonly payments: NamespaceClient;
  readonly feedback: NamespaceClient;
  readonly teams: NamespaceClient;
  readonly channels: NamespaceClient;
  readonly mascots: NamespaceClient;
  readonly admin: NamespaceClient;
  readonly announcements: NamespaceClient;
  readonly coupons: NamespaceClient;
  readonly invite: NamespaceClient;
  readonly investors: NamespaceClient;
  readonly referral: NamespaceClient;
  readonly rewards: NamespaceClient;
  readonly webhooks: NamespaceClient;

  constructor(options: TinyHumansClientOptions) {
    this.raw = new HttpClient(options);
    this.apiKeys = new NamespaceClient(this.raw, "/api-keys");
    this.auth = new NamespaceClient(this.raw, "/auth");
    this.inference = new NamespaceClient(this.raw, "/openai");
    this.agentIntegrations = new NamespaceClient(this.raw, "/agent-integrations");
    this.payments = new NamespaceClient(this.raw, "/payments");
    this.feedback = new NamespaceClient(this.raw, "/feedback");
    this.teams = new NamespaceClient(this.raw, "/teams");
    this.channels = new NamespaceClient(this.raw, "/channels");
    this.mascots = new NamespaceClient(this.raw, "/mascots");
    this.admin = new NamespaceClient(this.raw, "/admin");
    this.announcements = new NamespaceClient(this.raw, "/announcements");
    this.coupons = new NamespaceClient(this.raw, "/coupons");
    this.invite = new NamespaceClient(this.raw, "/invite");
    this.investors = new NamespaceClient(this.raw, "/investors");
    this.referral = new NamespaceClient(this.raw, "/referral");
    this.rewards = new NamespaceClient(this.raw, "/rewards");
    this.webhooks = new NamespaceClient(this.raw, "/webhooks");
  }

  health<T = unknown>(): Promise<T> {
    return this.raw.get<T>("/");
  }

  swagger<T = unknown>(): Promise<T> {
    return this.raw.get<T>("/swagger.json", { unwrapEnvelope: false });
  }
}
