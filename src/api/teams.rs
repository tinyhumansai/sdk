//! Team-scoped membership and access: listing teams, usage insights, invites,
//! joining/leaving/switching teams, member management, and billing plan helpers.

use reqwest::Method;
use serde_json::Value;

use super::types::{
    BillingPortalRequest, CodeRequest, CreateTeamInviteRequest, DynamicResponse,
    EmailInviteRequest, PurchasePlanRequest,
};
use crate::{enc, Error, HttpClient};

/// Typed client for the `/teams/*` routes.
pub struct TeamsApi<'a> {
    http: &'a HttpClient,
}

impl<'a> TeamsApi<'a> {
    pub fn new(http: &'a HttpClient) -> Self {
        Self { http }
    }

    /// List all teams the authenticated user belongs to, with their role in each.
    pub async fn list_teams(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/teams", &[], None, true)
            .await
    }

    /// Join a team using an invite code.
    pub async fn join_team(&self, request: &CodeRequest) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("join team request is serializable");
        self.http
            .send_typed(Method::POST, "/teams/join", &[], Some(&body), true)
            .await
    }

    /// Get usage and spend insights plus remaining budget for the current cycle.
    pub async fn get_my_usage(&self) -> Result<DynamicResponse, Error> {
        self.http
            .send_typed(Method::GET, "/teams/me/usage", &[], None, true)
            .await
    }

    /// Get details for a single team.
    pub async fn get_team(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}", enc(team_id));
        self.http
            .send_typed(Method::GET, &path, &[], None, true)
            .await
    }

    /// Get the team's current subscription plan.
    pub async fn get_billing_plan(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/billing/plan", enc(team_id));
        self.http
            .send_typed(Method::GET, &path, &[], None, true)
            .await
    }

    /// Create a Stripe billing portal session for the team.
    pub async fn create_billing_portal(
        &self,
        team_id: &str,
        request: &BillingPortalRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("billing portal request is serializable");
        let path = format!("/teams/{}/billing/portal", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// Purchase a subscription plan for the team (creates a checkout session).
    pub async fn purchase_plan(
        &self,
        team_id: &str,
        request: &PurchasePlanRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("purchase plan request is serializable");
        let path = format!("/teams/{}/billing/purchase", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// Create a shareable team invite.
    pub async fn create_invite(
        &self,
        team_id: &str,
        request: &CreateTeamInviteRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("team invite request is serializable");
        let path = format!("/teams/{}/invites", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// List a team's invites.
    pub async fn list_invites(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/invites", enc(team_id));
        self.http
            .send_typed(Method::GET, &path, &[], None, true)
            .await
    }

    /// Send a team invite via email.
    pub async fn send_email_invite(
        &self,
        team_id: &str,
        request: &EmailInviteRequest,
    ) -> Result<DynamicResponse, Error> {
        let body = serde_json::to_value(request).expect("email invite request is serializable");
        let path = format!("/teams/{}/invites/email", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], Some(&body), true)
            .await
    }

    /// Revoke a team invite.
    pub async fn revoke_invite(
        &self,
        team_id: &str,
        invite_id: &str,
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/invites/{}", enc(team_id), enc(invite_id));
        self.http
            .send_typed(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// Leave a team.
    pub async fn leave_team(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/leave", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], None, true)
            .await
    }

    /// List a team's members.
    pub async fn list_members(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/members", enc(team_id));
        self.http
            .send_typed(Method::GET, &path, &[], None, true)
            .await
    }

    /// Switch the user's active team.
    pub async fn switch_team(&self, team_id: &str) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/switch", enc(team_id));
        self.http
            .send_typed(Method::POST, &path, &[], None, true)
            .await
    }

    /// Update team settings.
    ///
    /// Requires the team-admin role — a role within this team, not platform
    /// administrator rights.
    pub async fn update_team(
        &self,
        team_id: &str,
        request: &Value,
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}", enc(team_id));
        self.http
            .send_typed(Method::PUT, &path, &[], Some(request), true)
            .await
    }

    /// Remove a member from the team.
    ///
    /// Requires the team-admin role — a role within this team, not platform
    /// administrator rights.
    pub async fn remove_member(
        &self,
        team_id: &str,
        user_id: &str,
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/members/{}", enc(team_id), enc(user_id));
        self.http
            .send_typed(Method::DELETE, &path, &[], None, true)
            .await
    }

    /// Change a member's role within the team.
    ///
    /// Requires the team-admin role — a role within this team, not platform
    /// administrator rights.
    pub async fn update_member_role(
        &self,
        team_id: &str,
        user_id: &str,
        request: &Value,
    ) -> Result<DynamicResponse, Error> {
        let path = format!("/teams/{}/members/{}/role", enc(team_id), enc(user_id));
        self.http
            .send_typed(Method::PUT, &path, &[], Some(request), true)
            .await
    }
}
