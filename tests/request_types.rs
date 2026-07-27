use serde_json::json;
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
        tool_results: vec![json!({"tool": "search", "result": []})],
    };
    assert_eq!(
        serde_json::to_value(continuation).unwrap()["cycleId"],
        "cycle-1"
    );
}
