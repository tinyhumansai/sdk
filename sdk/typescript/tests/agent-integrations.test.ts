import { describe, expect, it } from "vitest";

import { AgentIntegrationsApi } from "../src/api/agent-integrations.js";
import { mockClient } from "./helpers.js";

function makeApi(spec = { data: {} }) {
  const mock = mockClient(spec);
  return { api: new AgentIntegrationsApi(mock.http), ...mock };
}

describe("AgentIntegrationsApi", () => {
  // --- Apify ---

  it("runApifyActor posts to the run path with the body", async () => {
    const { api, last } = makeApi();
    await api.runApifyActor({ actorId: "a~b", input: { url: "x" } });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/apify/run");
    expect(last().body).toEqual({ actorId: "a~b", input: { url: "x" } });
  });

  it("getApifyRun encodes the runId", async () => {
    const { api, last } = makeApi();
    await api.getApifyRun("run/1");
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/apify/runs/run%2F1");
  });

  it("getApifyRunResults passes query params", async () => {
    const { api, last } = makeApi();
    await api.getApifyRunResults("run 2", { limit: 10, offset: 5 });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/apify/runs/run%202/results");
    expect(last().query.limit).toEqual(["10"]);
    expect(last().query.offset).toEqual(["5"]);
  });

  // --- Composio ---

  it("authorizeComposio posts to authorize", async () => {
    const { api, last } = makeApi();
    await api.authorizeComposio({ toolkit: "gmail" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/composio/authorize");
    expect(last().body).toEqual({ toolkit: "gmail" });
  });

  it("listComposioConnections gets connections", async () => {
    const { api, last } = makeApi();
    await api.listComposioConnections();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/composio/connections");
  });

  it("deleteComposioConnection encodes the id", async () => {
    const { api, last } = makeApi();
    await api.deleteComposioConnection("conn/9");
    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/agent-integrations/composio/connections/conn%2F9");
  });

  it("executeComposioTool posts to execute", async () => {
    const { api, last } = makeApi();
    await api.executeComposioTool({ tool: "GMAIL_SEND_EMAIL", arguments: { to: "x" } });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/composio/execute");
  });

  it("listComposioToolkits gets toolkits", async () => {
    const { api, last } = makeApi();
    await api.listComposioToolkits();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/composio/toolkits");
  });

  it("listComposioTools gets tools with filters", async () => {
    const { api, last } = makeApi();
    await api.listComposioTools({ toolkits: "gmail", tags: "email" });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/composio/tools");
    expect(last().query.toolkits).toEqual(["gmail"]);
    expect(last().query.tags).toEqual(["email"]);
  });

  it("listComposioTriggers gets triggers", async () => {
    const { api, last } = makeApi();
    await api.listComposioTriggers({ toolkit: "gmail" });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/composio/triggers");
    expect(last().query.toolkit).toEqual(["gmail"]);
  });

  it("createComposioTrigger posts to triggers", async () => {
    const { api, last } = makeApi();
    await api.createComposioTrigger({ connectionId: "c1", slug: "S" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/composio/triggers");
  });

  it("listComposioAvailableTriggers gets available", async () => {
    const { api, last } = makeApi();
    await api.listComposioAvailableTriggers({ toolkit: "github" });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/composio/triggers/available");
    expect(last().query.toolkit).toEqual(["github"]);
  });

  it("deleteComposioTrigger encodes the id", async () => {
    const { api, last } = makeApi();
    await api.deleteComposioTrigger("trig/1");
    expect(last().method).toBe("DELETE");
    expect(last().path).toBe("/agent-integrations/composio/triggers/trig%2F1");
  });

  // --- Crypto ---

  it("cryptoBridge posts to bridge", async () => {
    const { api, last } = makeApi();
    await api.cryptoBridge({
      srcChainId: 1,
      srcChainTokenIn: "0x0",
      srcChainTokenInAmount: "1",
      dstChainId: 137,
      dstChainTokenOut: "0x1",
      dstChainTokenOutRecipient: "0x2",
      srcChainOrderAuthorityAddress: "0x3",
      dstChainOrderAuthorityAddress: "0x4",
    });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/crypto/bridge");
  });

  it("listCryptoRoutes gets routes", async () => {
    const { api, last } = makeApi();
    await api.listCryptoRoutes();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/crypto/routes");
  });

  it("cryptoSwap posts to swap", async () => {
    const { api, last } = makeApi();
    await api.cryptoSwap({
      chainId: 1,
      tokenIn: "0x0",
      tokenInAmount: "1",
      tokenOut: "0x1",
      tokenOutRecipient: "0x2",
      senderAddress: "0x3",
    });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/crypto/swap");
  });

  // --- Financial APIs ---

  it("financialApisCommodity posts to commodity", async () => {
    const { api, last } = makeApi();
    await api.financialApisCommodity({ commodity: "WTI" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/financial-apis/commodity");
  });

  it("financialApisCryptoSeries posts to crypto-series", async () => {
    const { api, last } = makeApi();
    await api.financialApisCryptoSeries({ symbol: "BTC" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/financial-apis/crypto-series");
  });

  it("financialApisExchangeRate posts to exchange-rate", async () => {
    const { api, last } = makeApi();
    await api.financialApisExchangeRate({ fromCurrency: "BTC", toCurrency: "USD" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/financial-apis/exchange-rate");
  });

  it("financialApisOptions posts to options", async () => {
    const { api, last } = makeApi();
    await api.financialApisOptions({ symbol: "AAPL" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/financial-apis/options");
  });

  it("financialApisQuote posts to quote", async () => {
    const { api, last } = makeApi();
    await api.financialApisQuote({ symbol: "AAPL" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/financial-apis/quote");
  });

  // --- Google Places ---

  it("googlePlacesDetails posts to details", async () => {
    const { api, last } = makeApi();
    await api.googlePlacesDetails({ placeId: "p1" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/google-places/details");
  });

  it("googlePlacesSearch posts to search", async () => {
    const { api, last } = makeApi();
    await api.googlePlacesSearch({ query: "coffee" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/google-places/search");
  });

  // --- Media Generation ---

  it("mediaGenerationImages posts to images", async () => {
    const { api, last } = makeApi();
    await api.mediaGenerationImages({ prompt: "a cat" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/media-generation/images");
  });

  it("listMediaGenerationModels gets models with query", async () => {
    const { api, last } = makeApi();
    await api.listMediaGenerationModels({ includeUpstream: true });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/media-generation/models");
    expect(last().query.includeUpstream).toEqual(["true"]);
  });

  it("getMediaGenerationRequest encodes the requestId", async () => {
    const { api, last } = makeApi();
    await api.getMediaGenerationRequest("req/1");
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/media-generation/requests/req%2F1");
  });

  it("mediaGenerationVideos posts to videos", async () => {
    const { api, last } = makeApi();
    await api.mediaGenerationVideos({ prompt: "a dog running" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/media-generation/videos");
  });

  // --- Parallel ---

  it("parallelChat posts to chat", async () => {
    const { api, last } = makeApi();
    await api.parallelChat({ model: "base", messages: [{ role: "user", content: "hi" }] });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/chat");
  });

  it("parallelDataset posts to dataset", async () => {
    const { api, last } = makeApi();
    await api.parallelDataset({
      objective: "o",
      entityType: "company",
      matchConditions: [{ name: "x" }],
    });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/dataset");
  });

  it("getParallelDataset encodes findallId", async () => {
    const { api, last } = makeApi();
    await api.getParallelDataset("fa/1");
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/parallel/dataset/fa%2F1");
  });

  it("getParallelDatasetResult encodes findallId", async () => {
    const { api, last } = makeApi();
    await api.getParallelDatasetResult("fa 2");
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/parallel/dataset/fa%202/result");
  });

  it("parallelEnrich posts to enrich", async () => {
    const { api, last } = makeApi();
    await api.parallelEnrich({ input: "x", processor: "base", outputSchema: {} });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/enrich");
  });

  it("parallelExtract posts to extract", async () => {
    const { api, last } = makeApi();
    await api.parallelExtract({ urls: ["https://x.com"] });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/extract");
  });

  it("parallelResearch posts to research", async () => {
    const { api, last } = makeApi();
    await api.parallelResearch({ input: "x", processor: "core" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/research");
  });

  it("getParallelResearch encodes runId", async () => {
    const { api, last } = makeApi();
    await api.getParallelResearch("run/1");
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/parallel/research/run%2F1");
  });

  it("getParallelResearchResult encodes runId and passes query", async () => {
    const { api, last } = makeApi();
    await api.getParallelResearchResult("run 2", { timeoutSeconds: 30 });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/parallel/research/run%202/result");
    expect(last().query.timeoutSeconds).toEqual(["30"]);
  });

  it("parallelSearch posts to search", async () => {
    const { api, last } = makeApi();
    await api.parallelSearch({ objective: "o", searchQueries: ["q"] });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/parallel/search");
  });

  // --- Pricing ---

  it("getPricing gets pricing", async () => {
    const { api, last } = makeApi();
    await api.getPricing();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/pricing");
  });

  // --- Recall Calendar ---

  it("connectRecallCalendar posts to connect", async () => {
    const { api, last } = makeApi();
    await api.connectRecallCalendar();
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/recall-calendar/connect");
  });

  it("disconnectRecallCalendar posts to disconnect", async () => {
    const { api, last } = makeApi();
    await api.disconnectRecallCalendar();
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/recall-calendar/disconnect");
  });

  it("listRecallCalendarMeetings gets meetings", async () => {
    const { api, last } = makeApi();
    await api.listRecallCalendarMeetings();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/recall-calendar/meetings");
  });

  it("recallCalendarOAuthComplete gets oauth-complete with query", async () => {
    const { api, last } = makeApi();
    await api.recallCalendarOAuthComplete({ t: "tok/1", status: "success" });
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/recall-calendar/oauth-complete");
    expect(last().query.t).toEqual(["tok/1"]);
    expect(last().query.status).toEqual(["success"]);
  });

  it("getRecallCalendarStatus gets status", async () => {
    const { api, last } = makeApi();
    await api.getRecallCalendarStatus();
    expect(last().method).toBe("GET");
    expect(last().path).toBe("/agent-integrations/recall-calendar/status");
  });

  // --- Tenor ---

  it("tenorSearch posts to search", async () => {
    const { api, last } = makeApi();
    await api.tenorSearch({ query: "cats" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/tenor/search");
  });

  // --- Twilio ---

  it("twilioCall posts to call", async () => {
    const { api, last } = makeApi();
    await api.twilioCall({ to: "+15551234567" });
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/twilio/call");
    expect(last().body).toEqual({ to: "+15551234567" });
  });

  it("twilioIncomingCallWebhook encodes userId", async () => {
    const { api, last } = makeApi();
    await api.twilioIncomingCallWebhook("user/1");
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/twilio/webhooks/incoming-call/user%2F1");
  });

  it("twilioStatusWebhook encodes userId", async () => {
    const { api, last } = makeApi();
    await api.twilioStatusWebhook("user 2");
    expect(last().method).toBe("POST");
    expect(last().path).toBe("/agent-integrations/twilio/webhooks/status/user%202");
  });

  // --- Envelope unwrap ---

  it("unwraps the success envelope by default", async () => {
    const { api } = makeApi({ data: { pricing: { apify: 1 } } });
    const result = await api.getPricing();
    expect(result).toEqual({ pricing: { apify: 1 } });
  });
});
