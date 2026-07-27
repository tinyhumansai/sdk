//! The user's redeemed coupons and coupon redemption.

use reqwest::Method;

use super::types::{CodeRequest, DynamicResponse};
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
    pub async fn list_my_coupons(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/coupons/me", &[], None, true)
            .await
    }

    /// Redeem a coupon code.
    pub async fn redeem_coupon(&self, request: &CodeRequest) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("coupon request is serializable");
        self.http
            .send_typed(Method::POST, "/coupons/redeem", &[], Some(&body), true)
            .await
    }
}
