#![allow(missing_docs)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Answers keyed by `tool_call_id`, each value being a map of question text to
/// answer, used as the request body of
/// [`crate::AgentContext::continue_conversation`] and
/// [`crate::AgentContext::continue_conversation_streamed`].
pub type AnswersByToolCall = HashMap<String, HashMap<String, String>>;

/// A Workspace the current account belongs to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Creation time, Unix timestamp in seconds
    #[serde(default)]
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    #[serde(default)]
    pub updated_at: i64,
}

/// Response for [`crate::AgentContext::workspaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    /// Workspaces the current account belongs to
    pub workspaces: Vec<Workspace>,
}

/// An Agent in a Workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent UID, used as the path parameter of
    /// [`crate::AgentContext::conversation`]
    pub uid: String,
    /// Agent name
    pub name: String,
    /// Agent description
    #[serde(default)]
    pub description: String,
    /// Agent mode, e.g. `chat`
    #[serde(default)]
    pub mode: String,
    /// Icon URL
    #[serde(default)]
    pub icon: String,
    /// Whether published; only published Agents can start conversations
    #[serde(default)]
    pub is_published: bool,
    /// Publish time, Unix timestamp in seconds; 0 if unpublished
    #[serde(default)]
    pub published_at: i64,
    /// Creation time, Unix timestamp in seconds
    #[serde(default)]
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    #[serde(default)]
    pub updated_at: i64,
}

/// Response for [`crate::AgentContext::agents`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsResponse {
    /// Agent list
    pub agents: Vec<Agent>,
    /// Total number of matching Agents
    #[serde(default)]
    pub total: i32,
}

/// Options for [`crate::AgentContext::agents`]
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetAgentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl GetAgentsOptions {
    /// Create a new `GetAgentsOptions`
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the page number, starts at 1
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn limit(self, limit: i32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }

    /// Fuzzy search by Agent name
    #[inline]
    #[must_use]
    pub fn name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
}

/// Final run status of a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    /// The run completed successfully
    Succeeded,
    /// The run is paused, waiting for
    /// [`crate::AgentContext::continue_conversation`]
    Interrupted,
    /// The run failed
    Failed,
    /// The run was stopped
    Stopped,
}

/// A source referenced by the answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Reference index
    #[serde(default)]
    pub index: i32,
    /// Reference title
    #[serde(default)]
    pub title: String,
    /// Reference URL
    #[serde(default)]
    pub url: String,
}

/// One question the Agent needs you to answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// Question text
    pub question: String,
    /// Options; empty means free-form answer
    #[serde(default)]
    pub options: Vec<QuestionOption>,
    /// Whether multiple options may be selected
    #[serde(default)]
    pub multi_select: bool,
}

/// One option of a [`Question`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    /// Option text
    #[serde(default)]
    pub description: String,
}

/// Present when a conversation run is interrupted, waiting for
/// [`crate::AgentContext::continue_conversation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interrupt {
    /// ID of the node that triggered the interrupt
    pub node_id: String,
    /// Tool call ID of this inquiry; used as the answer key when continuing
    pub tool_call_id: String,
    /// Questions you need to answer
    #[serde(default)]
    pub questions: Vec<Question>,
    /// ID of the paused message
    #[serde(default)]
    pub message_id: i64,
    /// ID of the owning conversation
    #[serde(default)]
    pub chat_id: i64,
}

/// Present when a conversation run failed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentError {
    /// Error code
    #[serde(default)]
    pub code: i32,
    /// Error message
    #[serde(default)]
    pub message: String,
}

/// Response for [`crate::AgentContext::conversation`],
/// [`crate::AgentContext::continue_conversation`], and the final result of the
/// streamed counterparts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationResponse {
    /// Conversation identifier, used for follow-up questions and
    /// troubleshooting
    pub chat_uid: String,
    /// Message ID of this round (as a string). Accepts a raw JSON number too,
    /// defensively — see [`ChatStartedPayload::message_id`].
    #[serde(deserialize_with = "crate::serde_utils::deserialize_string_or_int_as_string")]
    pub message_id: String,
    /// Final run status
    pub status: ConversationStatus,
    /// Final answer text; valid when `status` is `succeeded`
    #[serde(default)]
    pub answer: String,
    /// Sources referenced by the answer
    #[serde(default)]
    pub references: Option<Vec<Reference>>,
    /// Run duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Present only when `status` is `interrupted`
    #[serde(default)]
    pub interrupt: Option<Interrupt>,
    /// Present only when the run failed
    #[serde(default)]
    pub error: Option<AgentError>,
}

impl ConversationResponse {
    /// Build a [`ConversationResponse`] from a streamed conversation's parts —
    /// `chat_uid`/`message_id` captured from an earlier `chat_started` event
    /// (`None` if it was never observed) and the `workflow_finished` payload.
    pub(crate) fn from_stream_parts(
        started: Option<(String, String)>,
        payload: WorkflowFinishedPayload,
    ) -> Self {
        let (chat_uid, message_id) = started.unwrap_or_default();
        Self {
            chat_uid,
            message_id,
            status: payload.status,
            answer: payload.outputs.answer.unwrap_or_default(),
            references: payload.outputs.references,
            elapsed_time: payload.elapsed_time,
            interrupt: payload.outputs.interrupt,
            error: payload.outputs.error,
        }
    }
}

/// Payload of a `chat_started` SSE event
#[derive(Debug, Clone, Deserialize)]
pub struct ChatStartedPayload {
    /// Conversation identifier
    pub chat_uid: String,
    /// Message ID of this round. The docs' SSE example shows this as a raw JSON
    /// number here (unlike the blocking response's top-level `message_id`,
    /// which is a quoted string) — accept either.
    #[serde(deserialize_with = "crate::serde_utils::deserialize_string_or_int_as_string")]
    pub message_id: String,
}

/// Payload of a `message` SSE event
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MessagePayload {
    /// Incremental answer text
    #[serde(default)]
    pub text: String,
}

/// `outputs` of a `workflow_finished` SSE event.
///
/// ⚠️ The PR docs only show an example for the `succeeded` path (`answer`
/// only); `interrupt`/`error` here are a best-effort guess mirroring the
/// blocking response's sibling fields for the `interrupted`/`failed` cases and
/// are **unverified** — confirm against a live/sandbox call that actually
/// triggers an interruption or failure before relying on this shape.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkflowOutputs {
    /// Final answer text; present when the run succeeded
    #[serde(default)]
    pub answer: Option<String>,
    /// Sources referenced by the answer
    #[serde(default)]
    pub references: Option<Vec<Reference>>,
    /// Present only when `status` is `interrupted` (unverified, see struct
    /// docs)
    #[serde(default)]
    pub interrupt: Option<Interrupt>,
    /// Present only when the run failed (unverified, see struct docs)
    #[serde(default)]
    pub error: Option<AgentError>,
}

/// Payload of a `workflow_finished` SSE event
#[derive(Debug, Clone, Deserialize)]
pub struct WorkflowFinishedPayload {
    /// Final run status
    pub status: ConversationStatus,
    /// Run duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Run outputs
    #[serde(default)]
    pub outputs: WorkflowOutputs,
}

/// One event observed while streaming
/// [`crate::AgentContext::conversation_streamed`]
/// or [`crate::AgentContext::continue_conversation_streamed`]
#[derive(Debug, Clone)]
pub enum ConversationStreamEvent {
    /// The run has started
    ChatStarted(ChatStartedPayload),
    /// An incremental piece of the answer
    Message(MessagePayload),
    /// The run finished (succeeded, interrupted, failed, or stopped) — the last
    /// event of a stream
    WorkflowFinished(ConversationResponse),
    /// An event type not recognized by this SDK version, carried as raw JSON so
    /// callers aren't broken by future additions to the API
    Other(serde_json::Value),
}

#[cfg(test)]
mod tests {
    use super::*;

    // The `data` payload of the "Run succeeded" example from
    // https://open.longbridge.com/en/docs/ai/chat/conversation
    const SUCCEEDED_JSON: &str = r#"{
        "chat_uid": "ct_9f2c1a5b",
        "message_id": "42",
        "status": "succeeded",
        "answer": "Tesla (TSLA.US) recently...",
        "references": [
            { "index": 1, "title": "...", "url": "..." }
        ],
        "elapsed_time": 3.21
    }"#;

    // The `data` payload of the "Run interrupted" example from the same page.
    const INTERRUPTED_JSON: &str = r#"{
        "chat_uid": "ct_9f2c1a5b",
        "message_id": "43",
        "status": "interrupted",
        "answer": "",
        "references": null,
        "elapsed_time": 1.05,
        "interrupt": {
            "node_id": "n_ask_human",
            "tool_call_id": "call_abc123",
            "questions": [
                {
                    "question": "Which time range would you like to check?",
                    "options": [
                        { "description": "Past week" },
                        { "description": "Past month" }
                    ],
                    "multi_select": false
                }
            ],
            "message_id": 43,
            "chat_id": 1001
        }
    }"#;

    #[test]
    fn deserialize_succeeded_conversation_response() {
        let resp: ConversationResponse = serde_json::from_str(SUCCEEDED_JSON).unwrap();
        assert_eq!(resp.chat_uid, "ct_9f2c1a5b");
        assert_eq!(resp.message_id, "42");
        assert_eq!(resp.status, ConversationStatus::Succeeded);
        assert_eq!(resp.answer, "Tesla (TSLA.US) recently...");
        assert_eq!(resp.references.as_ref().unwrap().len(), 1);
        assert_eq!(resp.references.as_ref().unwrap()[0].index, 1);
        assert!((resp.elapsed_time - 3.21).abs() < f64::EPSILON);
        assert!(resp.interrupt.is_none());
        assert!(resp.error.is_none());
    }

    #[test]
    fn deserialize_interrupted_conversation_response() {
        let resp: ConversationResponse = serde_json::from_str(INTERRUPTED_JSON).unwrap();
        assert_eq!(resp.status, ConversationStatus::Interrupted);
        let interrupt = resp.interrupt.expect("interrupt");
        assert_eq!(interrupt.node_id, "n_ask_human");
        assert_eq!(interrupt.tool_call_id, "call_abc123");
        assert_eq!(interrupt.message_id, 43);
        assert_eq!(interrupt.chat_id, 1001);
        assert_eq!(interrupt.questions.len(), 1);
        assert_eq!(interrupt.questions[0].options.len(), 2);
        assert!(!interrupt.questions[0].multi_select);
    }

    #[test]
    fn deserialize_chat_started_payload_with_numeric_message_id() {
        // The SSE example's `chat_started` event encodes `message_id` as a raw
        // JSON number, unlike the blocking response's quoted string.
        let json = r#"{"chat_uid":"ct_9f2c1a5b","message_id":42}"#;
        let payload: ChatStartedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.chat_uid, "ct_9f2c1a5b");
        assert_eq!(payload.message_id, "42");
    }

    #[test]
    fn deserialize_message_payload() {
        let json = r#"{"text":"Tesla"}"#;
        let payload: MessagePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.text, "Tesla");
    }

    #[test]
    fn deserialize_workflow_finished_payload() {
        let json = r#"{"status":"succeeded","elapsed_time":3.21,"outputs":{"answer":"Tesla (TSLA.US) recently..."}}"#;
        let payload: WorkflowFinishedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.status, ConversationStatus::Succeeded);
        assert!((payload.elapsed_time - 3.21).abs() < f64::EPSILON);
        assert_eq!(
            payload.outputs.answer.as_deref(),
            Some("Tesla (TSLA.US) recently...")
        );

        let resp = ConversationResponse::from_stream_parts(
            Some(("ct_9f2c1a5b".to_string(), "42".to_string())),
            payload,
        );
        assert_eq!(resp.chat_uid, "ct_9f2c1a5b");
        assert_eq!(resp.message_id, "42");
        assert_eq!(resp.answer, "Tesla (TSLA.US) recently...");
    }

    #[test]
    fn deserialize_workspaces_response() {
        let json = r#"{
            "workspaces": [
                { "id": "1001", "name": "My Workspace", "created_at": 1742000000, "updated_at": 1742001000 }
            ]
        }"#;
        let resp: WorkspacesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.workspaces.len(), 1);
        assert_eq!(resp.workspaces[0].id, "1001");
    }

    #[test]
    fn deserialize_agents_response() {
        let json = r#"{
            "agents": [
                {
                    "uid": "ag_7d3f9b2c",
                    "name": "US Stock Analyst",
                    "description": "Answers US stock questions with market and fundamental data",
                    "mode": "chat",
                    "icon": "https://cdn.longbridge.com/icons/agent.png",
                    "is_published": true,
                    "published_at": 1742000000,
                    "created_at": 1741000000,
                    "updated_at": 1742001000
                }
            ],
            "total": 12
        }"#;
        let resp: AgentsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.total, 12);
        assert_eq!(resp.agents[0].uid, "ag_7d3f9b2c");
        assert!(resp.agents[0].is_published);
    }
}
