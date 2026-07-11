//! Payments: Stripe subscriptions, Coinbase Commerce charges, credit balance
//! and top-ups, transaction history, the customer portal, and auto-recharge.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/payments/*` routes.
pub struct PaymentsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> PaymentsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Create a Coinbase Commerce charge.
    pub async fn create_coinbase_charge(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/payments/coinbase/charge",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Get charge status.
    pub async fn get_coinbase_charge(
        &self,
        gateway_transaction_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/payments/coinbase/charge/{}", enc(gateway_transaction_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Get Stripe auto-recharge settings for top-up credits.
    pub async fn get_auto_recharge(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/auto-recharge",
                &[],
                None,
                true,
            )
            .await
    }

    /// Update Stripe auto-recharge settings for top-up credits.
    pub async fn update_auto_recharge(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::PATCH,
                "/payments/credits/auto-recharge",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// List saved Stripe cards for auto recharge.
    pub async fn list_auto_recharge_cards(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/auto-recharge/cards",
                &[],
                None,
                true,
            )
            .await
    }

    /// Create a Stripe SetupIntent for adding a saved card.
    pub async fn create_auto_recharge_card_setup_intent(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/payments/credits/auto-recharge/cards/setup-intent",
                &[],
                None,
                true,
            )
            .await
    }

    /// Update a saved Stripe card for auto recharge.
    pub async fn update_auto_recharge_card(
        &self,
        payment_method_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!(
            "/payments/credits/auto-recharge/cards/{}",
            enc(payment_method_id)
        );
        self.http
            .send(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// Delete a saved Stripe card for auto recharge.
    pub async fn delete_auto_recharge_card(&self, payment_method_id: &str) -> Result<Value, Error> {
        let path = format!(
            "/payments/credits/auto-recharge/cards/{}",
            enc(payment_method_id)
        );
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Get the current user's credit balance.
    pub async fn get_credit_balance(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/payments/credits/balance", &[], None, true)
            .await
    }

    /// Create a credit top-up payment.
    pub async fn create_credit_top_up(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/payments/credits/top-up",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Handle canceled credit top-up (callback).
    pub async fn get_credit_top_up_cancel(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/top-up/cancel",
                &[],
                None,
                true,
            )
            .await
    }

    /// Handle successful credit top-up (callback).
    pub async fn get_credit_top_up_success(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/top-up/success",
                query,
                None,
                true,
            )
            .await
    }

    /// Get paginated credit transaction history.
    pub async fn list_credit_transactions(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/credits/transactions",
                query,
                None,
                true,
            )
            .await
    }

    /// Public Stripe Checkout redirect target (browser hand-off).
    pub async fn get_stripe_checkout_return(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/stripe/checkout/return",
                query,
                None,
                true,
            )
            .await
    }

    /// Get current subscription plan for authenticated user.
    pub async fn get_current_plan(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/payments/stripe/currentPlan", &[], None, true)
            .await
    }

    /// Get all available subscription plans.
    pub async fn get_stripe_plans(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/payments/stripe/plans", &[], None, true)
            .await
    }

    /// Create a Stripe Customer Portal session.
    pub async fn create_stripe_portal_session(&self) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/payments/stripe/portal", &[], None, true)
            .await
    }

    /// Stripe Customer Portal return page.
    pub async fn get_stripe_portal_return(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/payments/stripe/portal/return",
                &[],
                None,
                true,
            )
            .await
    }

    /// Create a Stripe Checkout Session for subscription purchase.
    pub async fn purchase_stripe_plan(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/payments/stripe/purchasePlan",
                &[],
                Some(body),
                true,
            )
            .await
    }
}
