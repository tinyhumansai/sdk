//! Event-name constants for the current backend Socket.IO surface.
//!
//! These constants cover the complete event catalog on backend `main`.
//! [`SocketConnection`](super::SocketConnection) also accepts arbitrary event
//! strings so additive backend events remain usable before an SDK release.

/// Events emitted by an SDK client and handled by the backend.
pub mod outbound {
    pub const INTEGRATION_METADATA_SYNC: &str = "integration:metadata-sync";
    pub const CHANNEL_REPLY: &str = "channel:reply";
    pub const CHANNEL_REACT: &str = "channel:react";
    pub const WEBHOOK_RESPONSE: &str = "webhook:response";

    pub const AGENT_START: &str = "agent:start";
    pub const AGENT_TRANSCRIPT: &str = "agent:transcript";
    pub const AGENT_SAY: &str = "agent:say";
    pub const AGENT_STOP: &str = "agent:stop";

    pub const ORCH_EFFECT_RESULT: &str = "orch:effect:result";
    pub const ORCH_TOOL_RESULT: &str = "orch:tool_result";
    pub const ORCH_REGISTER_TOOLS: &str = "orch:register_tools";

    pub const MEDULLA_REGISTER_AGENTS: &str = "medulla:register_agents";
    pub const MEDULLA_REGISTER_WORKFLOWS: &str = "medulla:register_workflows";
    pub const MEDULLA_TASK_ENVELOPE: &str = "medulla:task_envelope";
    pub const MEDULLA_TASK_RESULT: &str = "medulla:task_result";
    pub const MEDULLA_CAPABILITIES_RESULT: &str = "medulla:capabilities_result";
    pub const MEDULLA_WORKFLOW_RESULT: &str = "medulla:workflow_result";

    pub const WEBRTC_START: &str = "webrtc:start";
    pub const WEBRTC_ANSWER: &str = "webrtc:answer";
    pub const WEBRTC_ICE: &str = "webrtc:ice";
    pub const WEBRTC_SPEAK_BEGIN: &str = "webrtc:speak:begin";
    pub const WEBRTC_SPEAK_CHUNK: &str = "webrtc:speak:chunk";
    pub const WEBRTC_SPEAK_END: &str = "webrtc:speak:end";
    pub const WEBRTC_TEST_VISEME: &str = "webrtc:test:viseme";

    pub const BOT_JOIN: &str = "bot:join";
    pub const BOT_LEAVE: &str = "bot:leave";
    pub const BOT_SPEAK: &str = "bot:speak";

    pub const TUNNEL_REGISTER: &str = "tunnel:register";
    pub const TUNNEL_CONNECT: &str = "tunnel:connect";
    pub const TUNNEL_FRAME: &str = "tunnel:frame";
}

/// Events emitted by the backend and received by an SDK client.
pub mod inbound {
    pub const READY: &str = "ready";
    pub const ERROR: &str = "error";
    pub const CHANNEL_MESSAGE: &str = "channel:message";
    pub const CHANNEL_MESSAGE_SENT: &str = "channel:message:sent";
    pub const WEBHOOK_REQUEST: &str = "webhook:request";
    /// A correlation id is appended to this prefix.
    pub const WEBHOOK_RESPONSE_PREFIX: &str = "webhook:response:";

    pub const AGENT_STARTED: &str = "agent:started";
    pub const AGENT_STOPPED: &str = "agent:stopped";
    pub const AGENT_ERROR: &str = "agent:error";
    pub const AGENT_THOUGHT_START: &str = "agent:thought:start";
    pub const AGENT_THOUGHT_CHUNK: &str = "agent:thought:chunk";
    pub const AGENT_THOUGHT_END: &str = "agent:thought:end";
    pub const AGENT_AUDIO_START: &str = "agent:audio:start";
    pub const AGENT_AUDIO_CHUNK: &str = "agent:audio:chunk";
    pub const AGENT_AUDIO_END: &str = "agent:audio:end";
    pub const AGENT_VIDEO_START: &str = "agent:video:start";
    pub const AGENT_VIDEO_CHUNK: &str = "agent:video:chunk";
    pub const AGENT_VIDEO_END: &str = "agent:video:end";

    pub const ORCH_EFFECT_SEND_DM: &str = "orch:effect:send_dm";
    pub const ORCH_EFFECT_EVICT: &str = "orch:effect:evict";
    pub const ORCH_TOOL_CALL: &str = "orch:tool_call";

    pub const MEDULLA_TASK_RUN: &str = "medulla:task_run";
    pub const MEDULLA_TASK_SEND: &str = "medulla:task_send";
    pub const MEDULLA_TASK_ABORT: &str = "medulla:task_abort";
    pub const MEDULLA_CAPABILITIES_REQUEST: &str = "medulla:capabilities_request";
    pub const MEDULLA_WORKFLOW_REQUEST: &str = "medulla:workflow_request";
    pub const MEDULLA_EVENT: &str = "medulla:event";

    pub const WEBRTC_OFFER: &str = "webrtc:offer";
    pub const WEBRTC_ICE: &str = "webrtc:ice";
    pub const WEBRTC_ERROR: &str = "webrtc:error";

    pub const BOT_JOINED: &str = "bot:joined";
    pub const BOT_LEFT: &str = "bot:left";
    pub const BOT_TRANSCRIPT: &str = "bot:transcript";
    pub const BOT_TRANSCRIPT_DELTA: &str = "bot:transcript_delta";
    pub const BOT_IN_CALL_REQUEST: &str = "bot:in_call_request";
    pub const BOT_SPEAK_DEFERRED: &str = "bot:speak_deferred";
    pub const BOT_ERROR: &str = "bot:error";

    pub const TUNNEL_FRAME: &str = "tunnel:frame";
    pub const TUNNEL_PEER_STATUS: &str = "tunnel:peer-status";
    pub const TUNNEL_EVICTED: &str = "tunnel:evicted";
}
