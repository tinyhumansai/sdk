from __future__ import annotations

from typing import Any, Literal, NotRequired, Required, TypedDict


class ApiKeyCreate(TypedDict, total=False):
    name: Required[str]
    scopes: NotRequired[list[Literal["read", "write", "inference"]]]
    allowedIps: NotRequired[list[str]]
    expiresAt: NotRequired[str]


class BudgetSeatCreate(TypedDict, total=False):
    plan: Required[str]
    priority: NotRequired[int]
    agentIds: NotRequired[list[str]]
    enabled: NotRequired[bool]


class BudgetSeatUpdate(TypedDict, total=False):
    priority: NotRequired[int]
    agentIds: NotRequired[list[str]]
    enabled: NotRequired[bool]


class WorkspaceProfile(TypedDict):
    workspace: str
    medullaMd: str


class MedullaSessionCreate(TypedDict, total=False):
    title: NotRequired[str]
    meta: NotRequired[dict[str, Any]]
    asyncDelegation: NotRequired[bool]
    flavor: NotRequired[Literal["tui", "openhuman"]]
    workspaceProfiles: NotRequired[list[WorkspaceProfile]]


class MedullaTaskCreate(TypedDict, total=False):
    title: Required[str]
    description: NotRequired[str]
    status: NotRequired[Literal["open", "inProgress", "done", "cancelled"]]
    recurrence: NotRequired[dict[str, Any]]


class MedullaTaskUpdate(TypedDict, total=False):
    title: NotRequired[str]
    description: NotRequired[str]
    status: NotRequired[Literal["open", "inProgress", "done", "cancelled"]]
    recurrence: NotRequired[dict[str, Any]]


class MedullaTaskSourceCreate(TypedDict, total=False):
    repository: Required[str]
    state: NotRequired[Literal["open", "closed", "all"]]
    labels: NotRequired[list[str]]
    filter: NotRequired[str]
    token: NotRequired[str]
    enabled: NotRequired[bool]


class FeedbackIngest(TypedDict, total=False):
    type: Required[Literal["feature", "bug"]]
    title: Required[str]
    body: Required[str]
    product: Required[Literal["backend", "opencompany", "openhuman", "medulla"]]
    origin: NotRequired[str]
    externalRef: NotRequired[str]


class OrchestrationEvent(TypedDict):
    seq: int
    role: Literal["user", "assistant", "system"]
    sender: str
    body: str
    ts: int
    kind: str


class OrchestrationEventIngest(TypedDict):
    protocol: int
    counterpartAgentId: str
    sessionId: str
    event: OrchestrationEvent


class OrchestrationTool(TypedDict, total=False):
    name: Required[str]
    description: Required[str]
    parameters: NotRequired[dict[str, Any]]


class OrchestrationRun(TypedDict, total=False):
    input: Required[str]
    sessionId: NotRequired[str]
    flavor: NotRequired[Literal["tui", "openhuman"]]
    tools: NotRequired[list[OrchestrationTool]]
    options: NotRequired[dict[str, Any]]


class ToolResult(TypedDict, total=False):
    id: Required[str]
    ok: Required[bool]
    result: NotRequired[Any]
    error: NotRequired[str]


class OrchestrationContinue(TypedDict, total=False):
    cycleId: Required[str]
    toolResults: NotRequired[list[ToolResult]]


class FileUpload(TypedDict, total=False):
    file: Required[str]
    visibility: NotRequired[Literal["public", "private"]]
    ttlDays: NotRequired[int]


class HistoryUpload(TypedDict):
    file: str
    agent: Literal["claude", "codex", "opencode"]


class OpenCompanyCreate(TypedDict, total=False):
    slug: Required[str]
    company: NotRequired[str]


class CustomDomain(TypedDict):
    domain: str
