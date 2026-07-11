//! Admin-only routes: analytics dashboards, announcements, investors, coupons,
//! mascots, user credit/subscription management, and audit logs. These routes
//! authenticate via a bearer token or an `x-admin-service-token` header.

use reqwest::Method;
use serde_json::Value;

use crate::{enc, Error, HttpClient, QueryParam};

/// Typed client for the `/admin/*` routes.
pub struct AdminApi<'a> {
    http: &'a HttpClient,
}

impl<'a> AdminApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    // Analytics

    /// Get admin activity analytics.
    pub async fn get_analytics_activity(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/analytics/activity", query, None, true)
            .await
    }

    /// Messaging channel analytics — volume, day-by-day series, top users.
    pub async fn get_analytics_channels(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/analytics/channels", query, None, true)
            .await
    }

    /// Dashboard summary — DAU, MAU, events, API performance, feature usage.
    pub async fn get_analytics_dashboard(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/analytics/dashboard", query, None, true)
            .await
    }

    /// Get backend event analytics.
    pub async fn get_analytics_backend_events(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/admin/analytics/events/backend",
                query,
                None,
                true,
            )
            .await
    }

    /// Recent paying users (subscriptions) and topups.
    pub async fn get_analytics_financials_details(&self) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/admin/analytics/financials/details",
                &[],
                None,
                true,
            )
            .await
    }

    /// Home dashboard metrics — signups, active users, prompts, tokens.
    pub async fn get_analytics_home(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/analytics/home", query, None, true)
            .await
    }

    /// Inference usage analytics — prompts, tokens, cost, revenue, top users.
    pub async fn get_analytics_inference_usage(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/admin/analytics/inference/usage",
                query,
                None,
                true,
            )
            .await
    }

    /// Active-user leaderboard ranked by active days, prompts, tokens, or cost.
    pub async fn get_analytics_leaderboard(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/admin/analytics/leaderboard",
                query,
                None,
                true,
            )
            .await
    }

    /// Provider credit/limit health snapshot.
    pub async fn get_analytics_provider_credits(
        &self,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        self.http
            .send(
                Method::GET,
                "/admin/analytics/providers/credits",
                query,
                None,
                true,
            )
            .await
    }

    // Announcements

    /// List announcements.
    pub async fn list_announcements(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/announcements", query, None, true)
            .await
    }

    /// Create an announcement.
    pub async fn create_announcement(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/admin/announcements", &[], Some(body), true)
            .await
    }

    /// Get an announcement by id.
    pub async fn get_announcement(&self, announcement_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/announcements/{}", enc(announcement_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Update an announcement.
    pub async fn update_announcement(
        &self,
        announcement_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!("/admin/announcements/{}", enc(announcement_id));
        self.http
            .send(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// Delete an announcement.
    pub async fn delete_announcement(&self, announcement_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/announcements/{}", enc(announcement_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    // Management

    /// List admin audit-log entries.
    pub async fn list_audit_logs(&self, query: &[QueryParam]) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/audit-logs", query, None, true)
            .await
    }

    // Coupons

    /// Create a coupon.
    pub async fn create_coupon(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/admin/coupons", &[], Some(body), true)
            .await
    }

    /// Bulk-create coupons.
    pub async fn bulk_create_coupons(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/admin/coupons/bulk", &[], Some(body), true)
            .await
    }

    /// Update a coupon's expiry/active flag.
    pub async fn update_coupon(&self, coupon_id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/admin/coupons/{}", enc(coupon_id));
        self.http
            .send(Method::PATCH, &path, &[], Some(body), true)
            .await
    }

    /// Deactivate a coupon.
    pub async fn delete_coupon(&self, coupon_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/coupons/{}", enc(coupon_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    // Investors

    /// Create an investor.
    pub async fn create_investor(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/admin/investors", &[], Some(body), true)
            .await
    }

    /// Get an investor by id.
    pub async fn get_investor(&self, investor_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/investors/{}", enc(investor_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Update an investor.
    pub async fn update_investor(&self, investor_id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/admin/investors/{}", enc(investor_id));
        self.http
            .send(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// Delete an investor.
    pub async fn delete_investor(&self, investor_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/investors/{}", enc(investor_id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    /// Get event analytics for an investor.
    pub async fn get_investor_analytics(
        &self,
        investor_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/admin/investors/{}/analytics", enc(investor_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// List events for an investor, paginated.
    pub async fn list_investor_events(
        &self,
        investor_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/admin/investors/{}/events", enc(investor_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    // Mascots

    /// List all mascots with provenance.
    pub async fn list_mascots(&self) -> Result<Value, Error> {
        self.http
            .send(Method::GET, "/admin/mascots", &[], None, true)
            .await
    }

    /// Upload a custom Rive mascot.
    pub async fn create_mascot(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(Method::POST, "/admin/mascots", &[], Some(body), true)
            .await
    }

    /// Update a custom mascot's metadata or binary.
    pub async fn update_mascot(&self, id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/admin/mascots/{}", enc(id));
        self.http
            .send(Method::PUT, &path, &[], Some(body), true)
            .await
    }

    /// Delete a custom mascot.
    pub async fn delete_mascot(&self, id: &str) -> Result<Value, Error> {
        let path = format!("/admin/mascots/{}", enc(id));
        self.http.send(Method::DELETE, &path, &[], None, true).await
    }

    // Users

    /// Add credits to multiple users.
    pub async fn bulk_grant_user_credits(&self, body: &Value) -> Result<Value, Error> {
        self.http
            .send(
                Method::POST,
                "/admin/users/credits/bulk",
                &[],
                Some(body),
                true,
            )
            .await
    }

    /// Get a user's profile and credit balance.
    pub async fn get_admin_user(&self, user_id: &str) -> Result<Value, Error> {
        let path = format!("/admin/users/{}", enc(user_id));
        self.http.send(Method::GET, &path, &[], None, true).await
    }

    /// Per-user messaging channel breakdown across telegram/discord/web.
    pub async fn get_user_channel_analytics(
        &self,
        user_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/analytics/channels", enc(user_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Per-user usage analytics — prompts, tokens, cost, tools.
    pub async fn get_user_usage_analytics(
        &self,
        user_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/analytics/usage", enc(user_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Add or deduct a user's credits.
    pub async fn grant_user_credits(&self, user_id: &str, body: &Value) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/credits", enc(user_id));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// List a user's credit transactions.
    pub async fn list_user_credit_transactions(
        &self,
        user_id: &str,
        query: &[QueryParam],
    ) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/credits/transactions", enc(user_id));
        self.http.send(Method::GET, &path, query, None, true).await
    }

    /// Grant a paid subscription plan.
    pub async fn grant_user_subscription(
        &self,
        user_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/subscription", enc(user_id));
        self.http
            .send(Method::POST, &path, &[], Some(body), true)
            .await
    }

    /// Cancel a user's subscription and demote to free.
    pub async fn cancel_user_subscription(
        &self,
        user_id: &str,
        body: &Value,
    ) -> Result<Value, Error> {
        let path = format!("/admin/users/{}/subscription", enc(user_id));
        self.http
            .send(Method::DELETE, &path, &[], Some(body), true)
            .await
    }
}
