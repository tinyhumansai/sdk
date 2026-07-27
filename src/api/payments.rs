//! Payments: Stripe subscriptions, Coinbase Commerce charges, credit balance
//! and top-ups, transaction history, the customer portal, and auto-recharge.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::types::{BillingPlan, DynamicResponse};
use crate::{enc, Error, HttpClient, QueryParam};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CoinbasePlan {
    Basic,
    Pro,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CoinbaseInterval {
    Annual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCoinbaseChargeRequest {
    pub plan: CoinbasePlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CoinbaseInterval>,
    #[serde(default, skip_serializing_if = "Map::is_empty")]
    pub metadata: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentGateway {
    Stripe,
    Coinbase,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreditTopUpRequest {
    pub amount_usd: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<PaymentGateway>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AutoRechargeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_usd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharge_amount_usd: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpdateAutoRechargeCardRequest {
    #[serde(flatten)]
    pub fields: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseStripePlanRequest {
    pub plan: BillingPlan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_code: Option<String>,
}

/// Typed client for the `/payments/*` routes.
pub struct PaymentsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> PaymentsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Create a Coinbase Commerce charge.
    pub async fn create_coinbase_charge(
        &self,
        request: &CreateCoinbaseChargeRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("Coinbase request is serializable");
        self.http
            .send(
                Method::POST,
                "/payments/coinbase/charge",
                &[],
                Some(&body),
                true,
            )
            .await
            .map(Into::into)
    }

    /// Get charge status.
    pub async fn get_coinbase_charge(
        &self,
        gateway_transaction_id: &str,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/payments/coinbase/charge/{}", enc(gateway_transaction_id));
        self.http
            .send(Method::GET, &path, query, None, true)
            .await
            .map(Into::into)
    }

    /// Get Stripe auto-recharge settings for top-up credits.
    pub async fn get_auto_recharge(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/auto-recharge",
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Update Stripe auto-recharge settings for top-up credits.
    pub async fn update_auto_recharge(
        &self,
        request: &AutoRechargeRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("auto recharge request is serializable");
        self.http
            .send(
                Method::PATCH,
                "/payments/credits/auto-recharge",
                &[],
                Some(&body),
                true,
            )
            .await
            .map(Into::into)
    }

    /// List saved Stripe cards for auto recharge.
    pub async fn list_auto_recharge_cards(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/auto-recharge/cards",
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Create a Stripe SetupIntent for adding a saved card.
    pub async fn create_auto_recharge_card_setup_intent(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::POST,
                "/payments/credits/auto-recharge/cards/setup-intent",
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Update a saved Stripe card for auto recharge.
    pub async fn update_auto_recharge_card(
        &self,
        payment_method_id: &str,
        request: &UpdateAutoRechargeCardRequest,
    ) -> Result<DynamicResponse, Error> {
        let body =
            serde_json::to_value(request).expect("auto recharge card request is serializable");
        let path = format!(
            "/payments/credits/auto-recharge/cards/{}",
            enc(payment_method_id)
        );
        self.http
            .send(Method::PATCH, &path, &[], Some(&body), true)
            .await
            .map(Into::into)
    }

    /// Delete a saved Stripe card for auto recharge.
    pub async fn delete_auto_recharge_card(
        &self,
        payment_method_id: &str,
    ) -> Result<DynamicResponse, Error> {
        let path = format!(
            "/payments/credits/auto-recharge/cards/{}",
            enc(payment_method_id)
        );
        self.http
            .send(Method::DELETE, &path, &[], None, true)
            .await
            .map(Into::into)
    }

    /// Get the current user's credit balance.
    pub async fn get_credit_balance(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/payments/credits/balance", &[], None, true)
            .await
            .map(Into::into)
    }

    /// Create a credit top-up payment.
    pub async fn create_credit_top_up(
        &self,
        request: &CreditTopUpRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("credit top-up request is serializable");
        self.http
            .send(
                Method::POST,
                "/payments/credits/top-up",
                &[],
                Some(&body),
                true,
            )
            .await
            .map(Into::into)
    }

    /// Handle canceled credit top-up (callback).
    pub async fn get_credit_top_up_cancel(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/top-up/cancel",
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Handle successful credit top-up (callback).
    pub async fn get_credit_top_up_success(
        &self,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/top-up/success",
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Get paginated credit transaction history.
    pub async fn list_credit_transactions(
        &self,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/transactions",
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Public Stripe Checkout redirect target (browser hand-off).
    pub async fn get_stripe_checkout_return(
        &self,
        query: &[QueryParam],
    ) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/stripe/checkout/return",
                query,
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Get current subscription plan for authenticated user.
    pub async fn get_current_plan(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/payments/stripe/currentPlan", &[], None, true)
            .await
            .map(Into::into)
    }

    /// Get all available subscription plans.
    pub async fn get_stripe_plans(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::GET, "/payments/stripe/plans", &[], None, true)
            .await
            .map(Into::into)
    }

    /// Create a Stripe Customer Portal session.
    pub async fn create_stripe_portal_session(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(Method::POST, "/payments/stripe/portal", &[], None, true)
            .await
            .map(Into::into)
    }

    /// Stripe Customer Portal return page.
    pub async fn get_stripe_portal_return(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/stripe/portal/return",
                &[],
                None,
                true,
            )
            .await
            .map(Into::into)
    }

    /// Create a Stripe Checkout Session for subscription purchase.
    pub async fn purchase_stripe_plan(
        &self,
        request: &PurchaseStripePlanRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("Stripe plan request is serializable");
        self.http
            .send(
                Method::POST,
                "/payments/stripe/purchasePlan",
                &[],
                Some(&body),
                true,
            )
            .await
            .map(Into::into)
    }
}
