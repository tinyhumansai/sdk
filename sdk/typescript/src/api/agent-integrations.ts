import type { HttpClient, RequestOptions } from "../http.js";

// --- Apify ---

export interface RunApifyActorBody {
  actorId: string;
  input: Record<string, unknown>;
  sync?: boolean;
  timeoutSecs?: number;
  memoryMbytes?: number;
}

export interface ApifyRunResultsQuery {
  limit?: number;
  offset?: number;
}

// --- Composio ---

export interface ComposioAuthorizeBody {
  /** Toolkit slug (e.g. `gmail`) */
  toolkit: string;
  config?: Record<string, unknown>;
  /** Optional override for the post-auth redirect URL. */
  callbackUrl?: string;
}

export interface ComposioExecuteBody {
  /** Tool slug (e.g. `GMAIL_SEND_EMAIL`) */
  tool: string;
  arguments?: Record<string, unknown>;
  /** Optional Composio connected account id to target when the user has multiple accounts. */
  connectionId?: string;
}

export interface ComposioToolkitsRefreshQuery {
  full?: boolean;
}

export interface ComposioToolsQuery {
  toolkits?: string;
  tags?: string;
}

export interface ComposioTriggersQuery {
  toolkit?: string;
}

export interface CreateComposioTriggerBody {
  connectionId: string;
  /** Trigger slug (e.g. `GMAIL_NEW_GMAIL_MESSAGE`) */
  slug: string;
  triggerConfig?: Record<string, unknown>;
}

export interface ComposioAvailableTriggersQuery {
  toolkit: string;
  connectionId?: string;
}

// --- Crypto ---

export interface CryptoBridgeBody {
  /** Source deBridge chain id */
  srcChainId: number;
  /** Source token address */
  srcChainTokenIn: string;
  /** Source amount in the token's smallest unit */
  srcChainTokenInAmount: string;
  /** Destination deBridge chain id */
  dstChainId: number;
  /** Destination token address */
  dstChainTokenOut: string;
  /** Destination amount, or "auto" for market rate (default "auto") */
  dstChainTokenOutAmount?: string;
  /** Address that receives the bridged tokens */
  dstChainTokenOutRecipient: string;
  /** Source-chain order authority (can cancel/patch the order) */
  srcChainOrderAuthorityAddress: string;
  /** Destination-chain order authority */
  dstChainOrderAuthorityAddress: string;
}

export interface CryptoSwapBody {
  /** deBridge chain id the swap executes on */
  chainId: number;
  /** Input token address (use the zero address for native) */
  tokenIn: string;
  /** Input amount in the token's smallest unit */
  tokenInAmount: string;
  /** Output token address */
  tokenOut: string;
  /** Address that receives the output tokens */
  tokenOutRecipient: string;
  /** Address sending the transaction */
  senderAddress: string;
  /** Slippage percent or "auto" (default "auto") */
  slippage?: string;
}

// --- Financial APIs ---

export interface FinancialCommodityBody {
  commodity: "WTI" | "BRENT" | "NATURAL_GAS";
  interval?: "daily" | "weekly" | "monthly";
  limit?: number;
}

export interface FinancialCryptoSeriesBody {
  /** Crypto symbol, e.g. BTC, ETH */
  symbol: string;
  /** Quote market, defaults to USD */
  market?: string;
  /** Max points to return (most recent first), default 30 */
  limit?: number;
}

export interface FinancialExchangeRateBody {
  /** Source currency (fiat or crypto, e.g. BTC, USD, EUR) */
  fromCurrency: string;
  /** Target currency */
  toCurrency: string;
}

export interface FinancialOptionsBody {
  symbol: string;
  /** Include greeks and implied volatility (premium) */
  requireGreeks?: boolean;
}

export interface FinancialQuoteBody {
  /** Stock or index ticker (e.g. AAPL, SPY) */
  symbol: string;
}

// --- Google Places ---

export interface GooglePlacesDetailsBody {
  /** Google Places place ID */
  placeId: string;
}

export interface GooglePlacesSearchBody {
  /** Text query used to search for places */
  query: string;
  /** Maximum number of places to return */
  maxResults?: number;
}

// --- Media Generation ---

export interface MediaGenerationImagesBody {
  /** Upstream model id. Defaults to seedream-4-0-250828. */
  model?: string;
  prompt: string;
  /** e.g. 1024x1024, 1536x1024 */
  size?: string;
  n?: number;
  inputImages?: string[];
  seed?: number;
  wait?: boolean;
  timeoutSeconds?: number;
}

export interface MediaGenerationModelsQuery {
  includeUpstream?: boolean;
}

export interface MediaGenerationVideosBody {
  /** Upstream model id. Defaults to seedance-1-0-pro-fast-251015. */
  model?: string;
  prompt: string;
  /** First-frame / reference image URL for image-to-video */
  inputImage?: string;
  durationSeconds?: number;
  /** e.g. 16:9, 9:16, 1:1 */
  aspectRatio?: string;
  negativePrompt?: string;
  seed?: number;
  wait?: boolean;
  timeoutSeconds?: number;
}

// --- Parallel ---

export interface ParallelChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

export interface ParallelChatBody {
  model: "speed" | "lite" | "base" | "core";
  messages: ParallelChatMessage[];
  responseFormat?: Record<string, unknown>;
}

export interface ParallelDatasetMatchCondition {
  name: string;
  description?: string;
}

export interface ParallelDatasetBody {
  objective: string;
  entityType: string;
  matchConditions: ParallelDatasetMatchCondition[];
  /** Defaults to "base" */
  generator?: "preview" | "base" | "core" | "pro";
  matchLimit?: number;
  metadata?: Record<string, unknown>;
}

export interface ParallelEnrichBody {
  input: string | Record<string, unknown>;
  processor: "lite" | "base" | "core" | "ultra";
  outputSchema: Record<string, unknown>;
  inputSchema?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
  timeoutSeconds?: number;
}

export interface ParallelExtractBody {
  urls: string[];
  objective?: string;
  fullContent?: boolean;
}

export interface ParallelResearchBody {
  input: string | Record<string, unknown>;
  processor: "lite" | "base" | "core" | "ultra";
  outputSchema?: Record<string, unknown>;
  inputSchema?: Record<string, unknown>;
  metadata?: Record<string, unknown>;
  /** Block until the run completes (up to timeoutSeconds) */
  wait?: boolean;
  timeoutSeconds?: number;
}

export interface ParallelResearchResultQuery {
  timeoutSeconds?: number;
}

export interface ParallelSearchBody {
  objective: string;
  searchQueries: string[];
  mode?: "one-shot" | "agentic" | "fast";
}

// --- Recall Calendar ---

export interface RecallOAuthCompleteQuery {
  status?: "success" | "error";
  t: string;
}

// --- Tenor ---

export interface TenorSearchBody {
  /** Search query for GIFs */
  query: string;
  /** Maximum number of GIFs to return */
  limit?: number;
  /** Content safety filter level */
  contentFilter?: "off" | "low" | "medium" | "high";
  /** Pagination cursor from a previous response */
  pos?: string;
}

// --- Twilio ---

export interface TwilioCallBody {
  to: string;
  message?: string;
  twiml?: string;
  url?: string;
}

/**
 * Agent integrations: Apify, Composio, crypto swaps/bridges, financial APIs,
 * Google Places, media generation, Parallel web research, Recall calendar,
 * Tenor GIFs, and Twilio voice.
 */
export class AgentIntegrationsApi {
  constructor(private readonly http: HttpClient) {}

  // --- Apify ---

  /** Run an Apify actor */
  runApifyActor<T = unknown>(body: RunApifyActorBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/apify/run", body, options);
  }

  /** Get status of an Apify actor run */
  getApifyRun<T = unknown>(runId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/apify/runs/${encodeURIComponent(runId)}`,
      options,
    );
  }

  /** Get results from a completed Apify actor run */
  getApifyRunResults<T = unknown>(
    runId: string,
    query?: ApifyRunResultsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/apify/runs/${encodeURIComponent(runId)}/results`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  // --- Composio ---

  /** Start a Composio OAuth connection flow */
  authorizeComposio<T = unknown>(
    body: ComposioAuthorizeBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/composio/authorize", body, options);
  }

  /** List the user's Composio connections */
  listComposioConnections<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/composio/connections", options);
  }

  /** Delete a Composio connection */
  deleteComposioConnection<T = unknown>(
    connectionId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/agent-integrations/composio/connections/${encodeURIComponent(connectionId)}`,
      options,
    );
  }

  /** Execute a Composio tool on behalf of the user */
  executeComposioTool<T = unknown>(
    body: ComposioExecuteBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/composio/execute", body, options);
  }

  /** List Composio toolkits available to users */
  listComposioToolkits<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/composio/toolkits", options);
  }

  /** Refresh the cached Composio toolkit catalog (admin) */
  refreshComposioToolkits<T = unknown>(
    query?: ComposioToolkitsRefreshQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      "/agent-integrations/composio/toolkits/refresh",
      undefined,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** List Composio tools as OpenAI function-call schemas */
  listComposioTools<T = unknown>(
    query?: ComposioToolsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/agent-integrations/composio/tools", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** List the user's currently enabled Composio triggers */
  listComposioTriggers<T = unknown>(
    query?: ComposioTriggersQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/agent-integrations/composio/triggers", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Enable a Composio trigger on one of the user's connections */
  createComposioTrigger<T = unknown>(
    body: CreateComposioTriggerBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/composio/triggers", body, options);
  }

  /** List triggers available for a toolkit */
  listComposioAvailableTriggers<T = unknown>(
    query: ComposioAvailableTriggersQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/agent-integrations/composio/triggers/available", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Disable (delete) a Composio trigger owned by the user */
  deleteComposioTrigger<T = unknown>(
    triggerId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.delete<T>(
      `/agent-integrations/composio/triggers/${encodeURIComponent(triggerId)}`,
      options,
    );
  }

  // --- Crypto ---

  /** Build a cross-chain bridge transaction */
  cryptoBridge<T = unknown>(body: CryptoBridgeBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/crypto/bridge", body, options);
  }

  /** List supported chains for cross-chain swaps and bridges */
  listCryptoRoutes<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/crypto/routes", options);
  }

  /** Build a single-chain swap transaction */
  cryptoSwap<T = unknown>(body: CryptoSwapBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/crypto/swap", body, options);
  }

  // --- Financial APIs ---

  /** Commodity / futures price series (WTI, BRENT, NATURAL_GAS) via Alpha Vantage */
  financialApisCommodity<T = unknown>(
    body: FinancialCommodityBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/financial-apis/commodity", body, options);
  }

  /** Daily OHLCV series for a digital currency via Alpha Vantage DIGITAL_CURRENCY_DAILY */
  financialApisCryptoSeries<T = unknown>(
    body: FinancialCryptoSeriesBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      "/agent-integrations/financial-apis/crypto-series",
      body,
      options,
    );
  }

  /** Realtime FX or crypto exchange rate via Alpha Vantage CURRENCY_EXCHANGE_RATE */
  financialApisExchangeRate<T = unknown>(
    body: FinancialExchangeRateBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      "/agent-integrations/financial-apis/exchange-rate",
      body,
      options,
    );
  }

  /** Realtime options chain for a symbol via Alpha Vantage REALTIME_OPTIONS */
  financialApisOptions<T = unknown>(
    body: FinancialOptionsBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/financial-apis/options", body, options);
  }

  /** Latest quote for a stock or index symbol via Alpha Vantage GLOBAL_QUOTE */
  financialApisQuote<T = unknown>(
    body: FinancialQuoteBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/financial-apis/quote", body, options);
  }

  // --- Google Places ---

  /** Get place details via Google Places API */
  googlePlacesDetails<T = unknown>(
    body: GooglePlacesDetailsBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/google-places/details", body, options);
  }

  /** Search places via Google Places API */
  googlePlacesSearch<T = unknown>(
    body: GooglePlacesSearchBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/google-places/search", body, options);
  }

  // --- Media Generation ---

  /** Generate or edit an image via GMI (Seedream / SeedEdit) */
  mediaGenerationImages<T = unknown>(
    body: MediaGenerationImagesBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/media-generation/images", body, options);
  }

  /** List curated media-generation models */
  listMediaGenerationModels<T = unknown>(
    query?: MediaGenerationModelsQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/agent-integrations/media-generation/models", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Poll a media-generation request */
  getMediaGenerationRequest<T = unknown>(
    requestId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/media-generation/requests/${encodeURIComponent(requestId)}`,
      options,
    );
  }

  /** Generate a video via GMI (Seedance / Veo) */
  mediaGenerationVideos<T = unknown>(
    body: MediaGenerationVideosBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/media-generation/videos", body, options);
  }

  // --- Parallel ---

  /** Chat with the web via Parallel Chat Completions */
  parallelChat<T = unknown>(body: ParallelChatBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/chat", body, options);
  }

  /** Generate a web dataset via Parallel FindAll */
  parallelDataset<T = unknown>(
    body: ParallelDatasetBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/dataset", body, options);
  }

  /** Get dataset (FindAll) run status */
  getParallelDataset<T = unknown>(findallId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/parallel/dataset/${encodeURIComponent(findallId)}`,
      options,
    );
  }

  /** Get dataset (FindAll) matched candidates snapshot */
  getParallelDatasetResult<T = unknown>(
    findallId: string,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/parallel/dataset/${encodeURIComponent(findallId)}/result`,
      options,
    );
  }

  /** Enrich web data with a structured output schema (synchronous) */
  parallelEnrich<T = unknown>(body: ParallelEnrichBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/enrich", body, options);
  }

  /** Extract content from URLs via Parallel API */
  parallelExtract<T = unknown>(
    body: ParallelExtractBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/extract", body, options);
  }

  /** Start a deep research task (Parallel Task API) */
  parallelResearch<T = unknown>(
    body: ParallelResearchBody,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/research", body, options);
  }

  /** Get deep research run status */
  getParallelResearch<T = unknown>(runId: string, options?: RequestOptions): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/parallel/research/${encodeURIComponent(runId)}`,
      options,
    );
  }

  /** Block on a deep research run until completion */
  getParallelResearchResult<T = unknown>(
    runId: string,
    query?: ParallelResearchResultQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>(
      `/agent-integrations/parallel/research/${encodeURIComponent(runId)}/result`,
      { ...options, query: { ...query, ...options?.query } },
    );
  }

  /** Web search via Parallel API */
  parallelSearch<T = unknown>(body: ParallelSearchBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/parallel/search", body, options);
  }

  // --- Pricing ---

  /** Get plan-aware pricing for all agent integrations */
  getPricing<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/pricing", options);
  }

  // --- Recall Calendar ---

  /** Start the Recall.ai Calendar V1 OAuth flow for Google Calendar */
  connectRecallCalendar<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      "/agent-integrations/recall-calendar/connect",
      undefined,
      options,
    );
  }

  /** Disconnect the user's Google Calendar from Recall */
  disconnectRecallCalendar<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.post<T>(
      "/agent-integrations/recall-calendar/disconnect",
      undefined,
      options,
    );
  }

  /** List upcoming meetings from the connected calendar */
  listRecallCalendarMeetings<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/recall-calendar/meetings", options);
  }

  /** OAuth landing page (public) */
  recallCalendarOAuthComplete<T = unknown>(
    query: RecallOAuthCompleteQuery,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.get<T>("/agent-integrations/recall-calendar/oauth-complete", {
      ...options,
      query: { ...query, ...options?.query },
    });
  }

  /** Get the user's Recall calendar connection status */
  getRecallCalendarStatus<T = unknown>(options?: RequestOptions): Promise<T> {
    return this.http.get<T>("/agent-integrations/recall-calendar/status", options);
  }

  // --- Tenor ---

  /** Search for GIFs via the Tenor API */
  tenorSearch<T = unknown>(body: TenorSearchBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/tenor/search", body, options);
  }

  // --- Twilio ---

  /** Make a call via Twilio */
  twilioCall<T = unknown>(body: TwilioCallBody, options?: RequestOptions): Promise<T> {
    return this.http.post<T>("/agent-integrations/twilio/call", body, options);
  }

  /** Twilio webhook for incoming calls */
  twilioIncomingCallWebhook<T = unknown>(
    userId: string,
    body?: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/agent-integrations/twilio/webhooks/incoming-call/${encodeURIComponent(userId)}`,
      body,
      options,
    );
  }

  /** Twilio webhook for call status updates */
  twilioStatusWebhook<T = unknown>(
    userId: string,
    body?: unknown,
    options?: RequestOptions,
  ): Promise<T> {
    return this.http.post<T>(
      `/agent-integrations/twilio/webhooks/status/${encodeURIComponent(userId)}`,
      body,
      options,
    );
  }
}
