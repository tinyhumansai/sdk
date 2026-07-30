use serde_json::json;
use tinyhumans_sdk::socket::medulla::{
    CapabilitiesRequest, RegisterWorkflows, TaskRun, WorkflowRequest, WorkflowRequestOp,
};
use tinyhumans_sdk::socket::{SocketEvent, SocketPayload};
use tinyhumans_sdk::{Error, TinyHumansClient};

#[test]
fn socket_event_decodes_typed_json_payloads() {
    let event = SocketEvent::new(
        "medulla:task_run",
        SocketPayload::Json(vec![json!({
            "taskId": "task-1",
            "cycleId": "cycle-1",
            "instruction": "ship it",
            "workflow": "release",
            "timeoutMs": 30000
        })]),
    );

    let task: TaskRun = event.decode().unwrap();
    assert_eq!(task.task_id, "task-1");
    assert_eq!(task.workflow.as_deref(), Some("release"));
}

#[test]
fn medulla_decoder_covers_task_capability_workflow_and_stream_events() {
    let cases = [
        (
            "medulla:task_run",
            json!({
                "taskId": "task-1",
                "cycleId": "cycle-1",
                "instruction": "run",
                "timeoutMs": 1000
            }),
        ),
        (
            "medulla:task_send",
            json!({"taskId": "task-1", "input": "more"}),
        ),
        ("medulla:task_abort", json!({"taskId": "task-1"})),
        (
            "medulla:capabilities_request",
            json!({"probeId": "probe-1", "agentId": "agent-1"}),
        ),
        (
            "medulla:workflow_request",
            json!({"requestId": "req-1", "op": "node_kinds", "kind": "shell"}),
        ),
        (
            "medulla:event",
            json!({
                "seq": 7,
                "at": 123,
                "sessionId": "session-1",
                "event": {"kind": "assistant_delta", "delta": "hi"}
            }),
        ),
    ];

    for (name, payload) in cases {
        let decoded = SocketEvent::new(name, SocketPayload::Json(vec![payload]))
            .decode_medulla()
            .unwrap();
        assert!(decoded.is_some(), "{name} was not decoded");
    }
}

#[test]
fn workflow_request_op_uses_backend_wire_names() {
    let request = WorkflowRequest {
        request_id: "req-1".into(),
        op: WorkflowRequestOp::NodeKinds,
        workflow_id: None,
        kind: Some("shell".into()),
        instruction: None,
        agent_id: None,
    };
    assert_eq!(
        serde_json::to_value(request).unwrap()["op"],
        json!("node_kinds")
    );
}

#[test]
fn workflow_registration_reuses_http_descriptor_contract() {
    let payload: RegisterWorkflows = serde_json::from_value(json!({
        "agentId": "agent-1",
        "workflows": [{
            "id": "release",
            "nodeCount": 3,
            "enabled": true
        }]
    }))
    .unwrap();
    assert_eq!(payload.workflows[0].node_count, 3);
}

#[test]
fn binary_socket_payload_is_not_misdecoded_as_json() {
    let event = SocketEvent::new("agent:audio:chunk", SocketPayload::Binary(vec![1, 2, 3]));
    let error = event.decode::<CapabilitiesRequest>().unwrap_err();
    assert!(matches!(error, Error::UnexpectedSocketPayload(_)));
}

#[tokio::test]
async fn socket_connection_requires_a_bearer_token() {
    let error = TinyHumansClient::new("https://api.tinyhumans.ai")
        .connect_socket()
        .await
        .unwrap_err();
    assert!(matches!(error, Error::MissingSocketToken));
}

#[test]
fn unknown_socket_events_remain_available_to_generic_consumers() {
    let event = SocketEvent::new("bot:joined", SocketPayload::Json(vec![json!({"ok": true})]));
    assert!(event.decode_medulla().unwrap().is_none());
    assert_eq!(event.name, "bot:joined");
}
