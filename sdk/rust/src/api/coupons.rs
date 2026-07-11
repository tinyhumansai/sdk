//! Coupons: admin creation and management, the user's redeemed coupons, and redemption.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/coupons/*` routes.
pub struct CouponsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> CouponsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// Create a coupon (admin only).
    pub async fn create_coupon(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/coupons/admin", &[], Some(body), true)
            .await
    }

    /// List all coupons (admin only).
    pub async fn list_coupons(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/coupons/admin", query, None, true)
            .await
    }

    /// Deactivate a coupon (admin only).
    pub async fn delete_coupon(&self, coupon_id: &str) -> Result<Value, Error> {
        let path = format!("/coupons/admin/{}", enc(coupon_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
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
