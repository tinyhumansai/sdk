from __future__ import annotations

from .http import HttpClient, Json

from .api.admin import AdminApi
from .api.agent_integrations import AgentIntegrationsApi
from .api.announcements import AnnouncementsApi
from .api.auth import AuthApi
from .api.channels import ChannelsApi
from .api.coupons import CouponsApi
from .api.feedback import FeedbackApi
from .api.health import HealthApi
from .api.inference import InferenceApi
from .api.investors import InvestorsApi
from .api.invite import InviteApi
from .api.mascots import MascotsApi
from .api.payments import PaymentsApi
from .api.redirect import RedirectApi
from .api.referral import ReferralApi
from .api.rewards import RewardsApi
from .api.teams import TeamsApi
from .api.webhooks import WebhooksApi


class TinyHumansClient:
    """Typed client for the TinyHumans backend.

    Each attribute is a namespace client exposing one method per deployed
    operation. ``raw`` is the escape hatch for routes not yet surfaced.
    """

    def __init__(
        self,
        *,
        base_url: str,
        token: str | None = None,
        api_key: str | None = None,
        admin_service_token: str | None = None,
        headers: dict[str, str] | None = None,
        unwrap_envelope: bool = True,
        timeout: float = 30.0,
    ) -> None:
        self.raw = HttpClient(
            base_url=base_url,
            token=token,
            api_key=api_key,
            admin_service_token=admin_service_token,
            headers=headers,
            unwrap_envelope=unwrap_envelope,
            timeout=timeout,
        )

        self.admin = AdminApi(self.raw)
        self.agent_integrations = AgentIntegrationsApi(self.raw)
        self.announcements = AnnouncementsApi(self.raw)
        self.auth = AuthApi(self.raw)
        self.channels = ChannelsApi(self.raw)
        self.coupons = CouponsApi(self.raw)
        self.feedback = FeedbackApi(self.raw)
        self.health = HealthApi(self.raw)
        self.inference = InferenceApi(self.raw)
        self.investors = InvestorsApi(self.raw)
        self.invite = InviteApi(self.raw)
        self.mascots = MascotsApi(self.raw)
        self.payments = PaymentsApi(self.raw)
        self.redirect = RedirectApi(self.raw)
        self.referral = ReferralApi(self.raw)
        self.rewards = RewardsApi(self.raw)
        self.teams = TeamsApi(self.raw)
        self.webhooks = WebhooksApi(self.raw)

    def swagger(self) -> Json:
        """Fetch the deployed OpenAPI document (not enveloped)."""
        return self.raw.get("/swagger.json", unwrap_envelope=False)
