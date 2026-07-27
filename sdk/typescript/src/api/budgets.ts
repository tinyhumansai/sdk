import type { HttpClient, RequestOptions } from "../http.js";

export interface CreateBudgetSeatBody {
  plan: string;
  priority?: number;
  agentIds?: string[];
  enabled?: boolean;
}

export type UpdateBudgetSeatBody = Partial<CreateBudgetSeatBody>;

export class BudgetsApi {
  constructor(private readonly http: HttpClient) {}

  getBudgets<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/budgets", options);
  }

  createSeat<T = unknown>(
    body: CreateBudgetSeatBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/budgets/seats", body, options);
  }

  updateSeat<T = unknown>(
    seatId: string,
    body: UpdateBudgetSeatBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.patch<T>(
      `/budgets/seats/${encodeURIComponent(seatId)}`,
      body,
      options,
    );
  }

  deleteSeat<T = unknown>(seatId: string, options?: RequestOptions): Promise<T> {
    return this.http.delete<T>(
      `/budgets/seats/${encodeURIComponent(seatId)}`,
      options,
    );
  }
}
