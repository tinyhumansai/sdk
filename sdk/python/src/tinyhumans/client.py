from __future__ import annotations

from .api.agent_integrations import AgentIntegrationsApi
from .api.announcements import AnnouncementsApi
from .api.api_keys import ApiKeysApi
from .api.auth import AuthApi
from .api.budgets import BudgetsApi
from .api.channels import ChannelsApi
from .api.coupons import CouponsApi
from .api.feedback import FeedbackApi
from .api.health import HealthApi
from .api.inference import InferenceApi
from .api.invite import InviteApi
from .api.mascots import MascotsApi
from .api.medulla import MedullaApi
from .api.opencompany import OpenCompanyApi
from .api.orchestration import OrchestrationApi
from .api.payments import PaymentsApi
from .api.redirect import RedirectApi
from .api.referral import ReferralApi
from .api.rewards import RewardsApi
from .api.teams import TeamsApi
from .api.webhooks import WebhooksApi
from .http import HttpClient, Json


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
        headers: dict[str, str] | None = None,
        unwrap_envelope: bool = True,
        timeout: float = 30.0,
    ) -> None:
        self.raw = HttpClient(
            base_url=base_url,
            token=token,
            api_key=api_key,
            headers=headers,
            unwrap_envelope=unwrap_envelope,
            timeout=timeout,
        )

        self.agent_integrations = AgentIntegrationsApi(self.raw)
        self.api_keys = ApiKeysApi(self.raw)
        self.announcements = AnnouncementsApi(self.raw)
        self.auth = AuthApi(self.raw)
        self.budgets = BudgetsApi(self.raw)
        self.channels = ChannelsApi(self.raw)
        self.coupons = CouponsApi(self.raw)
        self.feedback = FeedbackApi(self.raw)
        self.health = HealthApi(self.raw)
        self.inference = InferenceApi(self.raw)
        self.invite = InviteApi(self.raw)
        self.mascots = MascotsApi(self.raw)
        self.medulla = MedullaApi(self.raw)
        self.opencompany = OpenCompanyApi(self.raw)
        self.orchestration = OrchestrationApi(self.raw)
        self.payments = PaymentsApi(self.raw)
        self.redirect = RedirectApi(self.raw)
        self.referral = ReferralApi(self.raw)
        self.rewards = RewardsApi(self.raw)
        self.teams = TeamsApi(self.raw)
        self.webhooks = WebhooksApi(self.raw)

    def swagger(self) -> Json:
        """Fetch the deployed OpenAPI document (not enveloped)."""
        return self.raw.get("/swagger.json", unwrap_envelope=False)
