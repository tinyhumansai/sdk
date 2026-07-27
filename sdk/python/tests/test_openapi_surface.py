from __future__ import annotations

from collections.abc import Callable
from typing import Any

import pytest
from helpers import RecordingHttp

from tinyhumans.api.agent_integrations import AgentIntegrationsApi
from tinyhumans.api.api_keys import ApiKeysApi
from tinyhumans.api.budgets import BudgetsApi
from tinyhumans.api.feedback import FeedbackApi
from tinyhumans.api.medulla import MedullaApi
from tinyhumans.api.opencompany import OpenCompanyApi
from tinyhumans.api.orchestration import OrchestrationApi
from tinyhumans.api.rewards import RewardsApi

Case = tuple[type[Any], Callable[[Any], Any], str, str]

CASES: list[Case] = [
    (ApiKeysApi, lambda a: a.list(), "GET", "/api-keys"),
    (ApiKeysApi, lambda a: a.create({"name": "ci"}), "POST", "/api-keys"),
    (ApiKeysApi, lambda a: a.revoke("x/y"), "DELETE", "/api-keys/x%2Fy"),
    (BudgetsApi, lambda a: a.get(), "GET", "/budgets"),
    (BudgetsApi, lambda a: a.connect_seat({"plan": "pro"}), "POST", "/budgets/seats"),
    (BudgetsApi, lambda a: a.update_seat("s/1", {}), "PATCH", "/budgets/seats/s%2F1"),
    (BudgetsApi, lambda a: a.disconnect_seat("s/1"), "DELETE", "/budgets/seats/s%2F1"),
    (
        FeedbackApi,
        lambda a: a.ingest(
            {"type": "bug", "title": "t", "body": "b", "product": "backend"}
        ),
        "POST",
        "/feedback/ingest",
    ),
    (MedullaApi, lambda a: a.create_session({}), "POST", "/medulla/v1/sessions"),
    (MedullaApi, lambda a: a.list_sessions(), "GET", "/medulla/v1/sessions"),
    (MedullaApi, lambda a: a.get_session("1"), "GET", "/medulla/v1/sessions/1"),
    (MedullaApi, lambda a: a.delete_session("1"), "DELETE", "/medulla/v1/sessions/1"),
    (
        MedullaApi,
        lambda a: a.send_message("1", "hi"),
        "POST",
        "/medulla/v1/sessions/1/messages",
    ),
    (
        MedullaApi,
        lambda a: a.list_messages("1"),
        "GET",
        "/medulla/v1/sessions/1/messages",
    ),
    (MedullaApi, lambda a: a.list_events("1"), "GET", "/medulla/v1/sessions/1/events"),
    (
        MedullaApi,
        lambda a: a.abort_session("1"),
        "POST",
        "/medulla/v1/sessions/1/abort",
    ),
    (
        MedullaApi,
        lambda a: a.stream_session("1"),
        "GET",
        "/medulla/v1/sessions/1/stream",
    ),
    (MedullaApi, lambda a: a.get_roster(), "GET", "/medulla/v1/roster"),
    (
        MedullaApi,
        lambda a: a.get_routing_strategy(),
        "GET",
        "/medulla/v1/routing/strategy",
    ),
    (
        MedullaApi,
        lambda a: a.set_routing_strategy("manual"),
        "PUT",
        "/medulla/v1/routing/strategy",
    ),
    (MedullaApi, lambda a: a.list_tasks(), "GET", "/medulla/v1/tasks"),
    (MedullaApi, lambda a: a.create_task({"title": "t"}), "POST", "/medulla/v1/tasks"),
    (MedullaApi, lambda a: a.list_task_sources(), "GET", "/medulla/v1/tasks/sources"),
    (
        MedullaApi,
        lambda a: a.create_task_source({"repository": "o/r"}),
        "POST",
        "/medulla/v1/tasks/sources",
    ),
    (
        MedullaApi,
        lambda a: a.sync_task_source("1"),
        "POST",
        "/medulla/v1/tasks/sources/1/sync",
    ),
    (
        MedullaApi,
        lambda a: a.delete_task_source("1"),
        "DELETE",
        "/medulla/v1/tasks/sources/1",
    ),
    (MedullaApi, lambda a: a.update_task("1", {}), "PATCH", "/medulla/v1/tasks/1"),
    (MedullaApi, lambda a: a.delete_task("1"), "DELETE", "/medulla/v1/tasks/1"),
    (OpenCompanyApi, lambda a: a.list_instances(), "GET", "/opencompany/instances"),
    (
        OpenCompanyApi,
        lambda a: a.create_instance({"slug": "x"}),
        "POST",
        "/opencompany/instances",
    ),
    (
        OpenCompanyApi,
        lambda a: a.suspend("x"),
        "POST",
        "/opencompany/instances/x/suspend",
    ),
    (
        OpenCompanyApi,
        lambda a: a.resume("x"),
        "POST",
        "/opencompany/instances/x/resume",
    ),
    (
        OpenCompanyApi,
        lambda a: a.delete_instance("x"),
        "DELETE",
        "/opencompany/instances/x",
    ),
    (
        OpenCompanyApi,
        lambda a: a.set_custom_domain("x", {"domain": "x.test"}),
        "PUT",
        "/opencompany/instances/x/custom-domain",
    ),
    (
        OpenCompanyApi,
        lambda a: a.delete_custom_domain("x"),
        "DELETE",
        "/opencompany/instances/x/custom-domain",
    ),
    (
        OpenCompanyApi,
        lambda a: a.verify_custom_domain("x"),
        "POST",
        "/opencompany/instances/x/custom-domain/verify",
    ),
    (
        OrchestrationApi,
        lambda a: a.ingest_event(
            {
                "protocol": 1,
                "counterpartAgentId": "a",
                "sessionId": "s",
                "event": {
                    "seq": 1,
                    "role": "user",
                    "sender": "u",
                    "body": "b",
                    "ts": 1,
                    "kind": "message",
                },
            }
        ),
        "POST",
        "/orchestration/v1/events",
    ),
    (
        OrchestrationApi,
        lambda a: a.update_world_diff({}),
        "POST",
        "/orchestration/v1/world-diff",
    ),
    (
        OrchestrationApi,
        lambda a: a.get_world_diff("s"),
        "GET",
        "/orchestration/v1/world-diff",
    ),
    (
        OrchestrationApi,
        lambda a: a.list_sessions(),
        "GET",
        "/orchestration/v1/sessions",
    ),
    (
        OrchestrationApi,
        lambda a: a.list_messages("1"),
        "GET",
        "/orchestration/v1/sessions/1/messages",
    ),
    (
        OrchestrationApi,
        lambda a: a.get_session_state("1"),
        "GET",
        "/orchestration/v1/sessions/1/state",
    ),
    (
        OrchestrationApi,
        lambda a: a.run({"input": "hi"}),
        "POST",
        "/orchestration/v1/run",
    ),
    (
        OrchestrationApi,
        lambda a: a.continue_run({"cycleId": "c"}),
        "POST",
        "/orchestration/v1/run/continue",
    ),
    (RewardsApi, lambda a: a.claim("history"), "POST", "/rewards/claim"),
    (
        AgentIntegrationsApi,
        lambda a: a.upload_file({"file": "data"}),
        "POST",
        "/agent-integrations/file-storage/files",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.list_files(),
        "GET",
        "/agent-integrations/file-storage/files",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.get_file_storage_usage(),
        "GET",
        "/agent-integrations/file-storage/usage",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.get_file("1"),
        "GET",
        "/agent-integrations/file-storage/files/1",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.update_file_visibility("1", "public"),
        "PATCH",
        "/agent-integrations/file-storage/files/1",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.delete_file("1"),
        "DELETE",
        "/agent-integrations/file-storage/files/1",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.download_file("1"),
        "GET",
        "/agent-integrations/file-storage/files/1/download",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.create_file_link("1"),
        "POST",
        "/agent-integrations/file-storage/files/1/link",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.get_public_file("1"),
        "GET",
        "/agent-integrations/file-storage/public/1",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.upload_history({"file": "data", "agent": "codex"}),
        "POST",
        "/agent-integrations/history-rewards/uploads",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.claim_history_reward(),
        "POST",
        "/agent-integrations/history-rewards/claim",
    ),
    (
        AgentIntegrationsApi,
        lambda a: a.get_history_reward_status(),
        "GET",
        "/agent-integrations/history-rewards/status",
    ),
]


@pytest.mark.parametrize(("api_type", "invoke", "method", "path"), CASES)
def test_missing_openapi_route(
    api_type: type[Any], invoke: Callable[[Any], Any], method: str, path: str
) -> None:
    http = RecordingHttp({"data": {}})
    invoke(api_type(http))
    assert (http.last["method"], http.last["path"]) == (method, path)


def test_route_inventory_has_all_57_additions() -> None:
    assert len(CASES) == 57
