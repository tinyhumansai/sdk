use serde_json::json;
use tinyhumans_sdk::api::orchestration::ToolResult;
use tinyhumans_sdk::api::types::{
    BillingPlan, ContinueRunRequest, CreateFeedbackRequest, FeedbackType, OrchestrationEvent,
    OrchestrationEventRequest, OrchestrationRole, PurchasePlanRequest,
};

#[test]
fn documented_enums_use_backend_wire_values() {
    let feedback = CreateFeedbackRequest {
        kind: FeedbackType::Feature,
        title: "Typed SDK".into(),
        body: "Expose request models".into(),
    };
    assert_eq!(
        serde_json::to_value(feedback).unwrap(),
        json!({
            "type": "feature",
            "title": "Typed SDK",
            "body": "Expose request models"
        })
    );

    let purchase = PurchasePlanRequest {
        plan: BillingPlan::ProYearly,
        success_url: Some("https://example.test/success".into()),
        cancel_url: None,
    };
    assert_eq!(
        serde_json::to_value(purchase).unwrap(),
        json!({
            "plan": "PRO_YEARLY",
            "successUrl": "https://example.test/success"
        })
    );
}

#[test]
fn nested_orchestration_event_is_fully_typed() {
    let request = OrchestrationEventRequest {
        protocol: 1,
        counterpart_agent_id: "agent-2".into(),
        session_id: "session-1".into(),
        event: OrchestrationEvent {
            seq: 3,
            role: OrchestrationRole::Assistant,
            sender: "agent-1".into(),
            body: "done".into(),
            ts: 1_700_000_000,
            kind: "message".into(),
        },
    };

    assert_eq!(
        serde_json::to_value(request).unwrap()["event"]["role"],
        "assistant"
    );

    let continuation = ContinueRunRequest {
        cycle_id: "cycle-1".into(),
        tool_results: vec![ToolResult {
            id: "call-1".into(),
            ok: true,
            result: Some(json!({"tool": "search", "result": []})),
            error: None,
        }],
    };
    let encoded = serde_json::to_value(continuation).unwrap();
    assert_eq!(encoded["cycleId"], "cycle-1");
    // A successful result omits `error` rather than sending an explicit null.
    assert_eq!(encoded["toolResults"][0]["ok"], true);
    assert!(encoded["toolResults"][0].get("error").is_none());
}
