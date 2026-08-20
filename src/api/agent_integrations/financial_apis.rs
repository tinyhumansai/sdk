//! Alpha Vantage market data: quotes, FX, options, commodities, series.

use super::AgentIntegrationsApi;
use crate::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OptionsRequest {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_greeks: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateRequest {
    pub from_currency: String,
    pub to_currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CryptoSeriesRequest {
    pub symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Commodity {
    Wti,
    Brent,
    NaturalGas,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SeriesInterval {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommodityRequest {
    pub commodity: Commodity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<SeriesInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    #[serde(default)]
    pub open: f64,
    #[serde(default)]
    pub high: f64,
    #[serde(default)]
    pub low: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub previous_close: f64,
    #[serde(default)]
    pub change: f64,
    #[serde(default)]
    pub change_percent: String,
    #[serde(default)]
    pub latest_trading_day: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    #[serde(default)]
    pub quote: Quote,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRate {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    #[serde(default)]
    pub bid: Option<f64>,
    #[serde(default)]
    pub ask: Option<f64>,
    #[serde(default)]
    pub last_refreshed: String,
    #[serde(default)]
    pub time_zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateResponse {
    pub rate: ExchangeRateValue,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExchangeRateValue {
    Detailed(ExchangeRate),
    Numeric(f64),
}

impl Default for ExchangeRateValue {
    fn default() -> Self {
        Self::Numeric(0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OptionsResponse {
    pub symbol: String,
    #[serde(default)]
    pub contracts: Vec<Value>,
    #[serde(default)]
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SeriesPoint {
    pub date: String,
    #[serde(default)]
    pub open: f64,
    #[serde(default)]
    pub high: f64,
    #[serde(default)]
    pub low: f64,
    #[serde(default)]
    pub close: f64,
    #[serde(default)]
    pub volume: f64,
    #[serde(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FinancialSeries {
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub commodity: String,
    #[serde(default)]
    pub interval: String,
    #[serde(default)]
    pub unit: String,
    #[serde(default)]
    pub series: Vec<SeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FinancialSeriesResponse {
    #[serde(default)]
    pub series: FinancialSeries,
    #[serde(default)]
    pub cost_usd: f64,
}

impl AgentIntegrationsApi<'_> {
    /// Commodity / futures price series (WTI, BRENT, NATURAL_GAS) via Alpha Vantage.
    pub async fn financial_apis_commodity(
        &self,
        request: &impl Serialize,
    ) -> Result<FinancialSeriesResponse, Error> {
        self.post("/agent-integrations/financial-apis/commodity", request)
            .await
    }

    /// Daily OHLCV series for a digital currency via Alpha Vantage DIGITAL_CURRENCY_DAILY.
    pub async fn financial_apis_crypto_series(
        &self,
        request: &impl Serialize,
    ) -> Result<FinancialSeriesResponse, Error> {
        self.post("/agent-integrations/financial-apis/crypto-series", request)
            .await
    }

    /// Realtime FX or crypto exchange rate via Alpha Vantage CURRENCY_EXCHANGE_RATE.
    pub async fn financial_apis_exchange_rate(
        &self,
        request: &impl Serialize,
    ) -> Result<ExchangeRateResponse, Error> {
        self.post("/agent-integrations/financial-apis/exchange-rate", request)
            .await
    }

    /// Realtime options chain for a symbol via Alpha Vantage REALTIME_OPTIONS.
    pub async fn financial_apis_options(
        &self,
        request: &impl Serialize,
    ) -> Result<OptionsResponse, Error> {
        self.post("/agent-integrations/financial-apis/options", request)
            .await
    }

    /// Latest quote for a stock or index symbol via Alpha Vantage GLOBAL_QUOTE.
    pub async fn financial_apis_quote(
        &self,
        request: &impl Serialize,
    ) -> Result<QuoteResponse, Error> {
        self.post("/agent-integrations/financial-apis/quote", request)
            .await
    }
}
