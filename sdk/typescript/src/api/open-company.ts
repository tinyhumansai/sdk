import type { HttpClient, RequestOptions } from "../http.js";

export interface CreateOpenCompanyInstanceBody {
  slug: string;
  company?: string;
}

export interface DeleteOpenCompanyInstanceQuery {
  purge_data?: "true" | "false";
}

export interface SetOpenCompanyCustomDomainBody {
  domain: string;
}

export class OpenCompanyApi {
  constructor(private readonly http: HttpClient) {}

  listInstances<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/opencompany/instances", options);
  }

  createInstance<T = unknown>(
    body: CreateOpenCompanyInstanceBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/opencompany/instances", body, options);
  }

  getInstance<T = unknown>(slug: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}`,
      options,
    );
  }

  suspendInstance<T = unknown>(slug: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}/suspend`,
      undefined,
      options,
    );
  }

  resumeInstance<T = unknown>(slug: string, options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}/resume`,
      undefined,
      options,
    );
  }

  deleteInstance<T = unknown>(
    slug: string,
    query?: DeleteOpenCompanyInstanceQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  setCustomDomain<T = unknown>(
    slug: string,
    body: SetOpenCompanyCustomDomainBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.put<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}/custom-domain`,
      body,
      options,
    );
  }

  deleteCustomDomain<T = unknown>(
    slug: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}/custom-domain`,
      options,
    );
  }

  verifyCustomDomain<T = unknown>(
    slug: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/opencompany/instances/${encodeURIComponent(slug)}/custom-domain/verify`,
      undefined,
      options,
    );
  }
}
