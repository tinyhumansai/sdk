//! The user's redeemed coupons and coupon redemption.

use reqwest::Method;
use serde_json::Value;

use crate::{Error, HttpClient};

/// Typed client for the `/coupons/*` routes.
pub struct CouponsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> CouponsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List the current user's redeemed coupons.
    pub async fn list_my_coupons(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/coupons/me", &[], None, true)
            .await
    }

    /// Redeem a coupon code.
    pub async fn redeem_coupon(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/coupons/redeem", &[], Some(body), true)
            .await
    }
}
