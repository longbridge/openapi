//! AI Agent conversation types.
//!
//! These are plain output types (never constructed from Python), so — like
//! [`crate::sharelist::types`], the closest existing analog for a pure-HTTP
//! domain — they use `#[pyclass(get_all, skip_from_py_object)]` plus a manual
//! `From` conversion rather than the `#[derive(PyObject)]` macro used by
//! `trade::types`. The macro's field-shape support
//! (`#[py(array)]`/`#[py(opt)]`) doesn't cleanly cover `Option<Vec<T>>` (e.g.
//! `ConversationResponse::references`), and every field here is infallible to
//! convert (no `Decimal`/time parsing), so a plain `From` is simpler than
//! fighting the macro or introducing a `TryFrom` that can never actually fail.
use longbridge_python_macros::PyEnum;
use pyo3::pyclass;

use crate::fundamental::types::JsonValue;

/// A Workspace the current account belongs to
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<longbridge::agent::Workspace> for Workspace {
    fn from(v: longbridge::agent::Workspace) -> Self {
        Self {
            id: v.id,
            name: v.name,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Response for `AgentContext.workspaces`/`AsyncAgentContext.workspaces`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct WorkspacesResponse {
    pub workspaces: Vec<Workspace>,
}

impl From<longbridge::agent::WorkspacesResponse> for WorkspacesResponse {
    fn from(v: longbridge::agent::WorkspacesResponse) -> Self {
        Self {
            workspaces: v.workspaces.into_iter().map(Into::into).collect(),
        }
    }
}

/// An Agent in a Workspace
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Agent {
    pub uid: String,
    pub name: String,
    pub description: String,
    pub mode: String,
    pub icon: String,
    pub is_published: bool,
    pub published_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<longbridge::agent::Agent> for Agent {
    fn from(v: longbridge::agent::Agent) -> Self {
        Self {
            uid: v.uid,
            name: v.name,
            description: v.description,
            mode: v.mode,
            icon: v.icon,
            is_published: v.is_published,
            published_at: v.published_at,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Response for `AgentContext.agents`/`AsyncAgentContext.agents`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct AgentsResponse {
    pub agents: Vec<Agent>,
    pub total: i32,
}

impl From<longbridge::agent::AgentsResponse> for AgentsResponse {
    fn from(v: longbridge::agent::AgentsResponse) -> Self {
        Self {
            agents: v.agents.into_iter().map(Into::into).collect(),
            total: v.total,
        }
    }
}

/// Final run status of a conversation
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::agent::ConversationStatus")]
pub(crate) enum ConversationStatus {
    /// The run completed successfully
    Succeeded,
    /// The run is paused, waiting for `AgentContext.continue_conversation`
    Interrupted,
    /// The run failed
    Failed,
    /// The run was stopped
    Stopped,
}

/// A source referenced by the answer
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Reference {
    pub index: i32,
    pub title: String,
    pub url: String,
}

impl From<longbridge::agent::Reference> for Reference {
    fn from(v: longbridge::agent::Reference) -> Self {
        Self {
            index: v.index,
            title: v.title,
            url: v.url,
        }
    }
}

/// One option of a [`Question`]
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct QuestionOption {
    pub description: String,
}

impl From<longbridge::agent::QuestionOption> for QuestionOption {
    fn from(v: longbridge::agent::QuestionOption) -> Self {
        Self {
            description: v.description,
        }
    }
}

/// One question the Agent needs you to answer
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Question {
    pub question: String,
    pub options: Vec<QuestionOption>,
    pub multi_select: bool,
}

impl From<longbridge::agent::Question> for Question {
    fn from(v: longbridge::agent::Question) -> Self {
        Self {
            question: v.question,
            options: v.options.into_iter().map(Into::into).collect(),
            multi_select: v.multi_select,
        }
    }
}

/// Present when a conversation run is interrupted, waiting for
/// `AgentContext.continue_conversation`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct Interrupt {
    pub node_id: String,
    pub tool_call_id: String,
    pub questions: Vec<Question>,
    pub message_id: i64,
    pub chat_id: i64,
}

impl From<longbridge::agent::Interrupt> for Interrupt {
    fn from(v: longbridge::agent::Interrupt) -> Self {
        Self {
            node_id: v.node_id,
            tool_call_id: v.tool_call_id,
            questions: v.questions.into_iter().map(Into::into).collect(),
            message_id: v.message_id,
            chat_id: v.chat_id,
        }
    }
}

/// Present when a conversation run failed
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct AgentError {
    pub code: i32,
    pub message: String,
}

impl From<longbridge::agent::AgentError> for AgentError {
    fn from(v: longbridge::agent::AgentError) -> Self {
        Self {
            code: v.code,
            message: v.message,
        }
    }
}

/// Response for
/// `AgentContext.conversation`/`AgentContext.continue_conversation`,
/// and the final result of the streamed counterparts
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ConversationResponse {
    pub chat_uid: String,
    pub message_id: String,
    pub status: ConversationStatus,
    pub answer: String,
    pub references: Option<Vec<Reference>>,
    pub elapsed_time: f64,
    pub interrupt: Option<Interrupt>,
    pub error: Option<AgentError>,
}

impl From<longbridge::agent::ConversationResponse> for ConversationResponse {
    fn from(v: longbridge::agent::ConversationResponse) -> Self {
        Self {
            chat_uid: v.chat_uid,
            message_id: v.message_id,
            status: v.status.into(),
            answer: v.answer,
            references: v
                .references
                .map(|refs| refs.into_iter().map(Into::into).collect()),
            elapsed_time: v.elapsed_time,
            interrupt: v.interrupt.map(Into::into),
            error: v.error.map(Into::into),
        }
    }
}

/// Payload of a `chat_started` stream event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ChatStartedPayload {
    pub chat_uid: String,
    pub message_id: String,
}

impl From<longbridge::agent::ChatStartedPayload> for ChatStartedPayload {
    fn from(v: longbridge::agent::ChatStartedPayload) -> Self {
        Self {
            chat_uid: v.chat_uid,
            message_id: v.message_id,
        }
    }
}

/// Payload of a `message` stream event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct MessagePayload {
    pub text: String,
}

impl From<longbridge::agent::MessagePayload> for MessagePayload {
    fn from(v: longbridge::agent::MessagePayload) -> Self {
        Self { text: v.text }
    }
}

/// `inputs` of a `workflow_started` stream event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct WorkflowStartedInputs {
    pub chat_id: i64,
    pub chat_uid: String,
    pub message_id: String,
    pub query: String,
}

impl From<longbridge::agent::WorkflowStartedInputs> for WorkflowStartedInputs {
    fn from(v: longbridge::agent::WorkflowStartedInputs) -> Self {
        Self {
            chat_id: v.chat_id,
            chat_uid: v.chat_uid,
            message_id: v.message_id,
            query: v.query,
        }
    }
}

/// Payload of a `workflow_started` stream event, observed right after
/// `chat_started`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct WorkflowStartedPayload {
    pub hit_cache: bool,
    pub inputs: WorkflowStartedInputs,
    pub started_at: i64,
    pub workflow_id: i64,
}

impl From<longbridge::agent::WorkflowStartedPayload> for WorkflowStartedPayload {
    fn from(v: longbridge::agent::WorkflowStartedPayload) -> Self {
        Self {
            hit_cache: v.hit_cache,
            inputs: v.inputs.into(),
            started_at: v.started_at,
            workflow_id: v.workflow_id,
        }
    }
}

/// Payload of a `chat_finished` stream event, observed once all `message`
/// events for this round have been sent, shortly before `workflow_finished`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ChatFinishedPayload {
    pub chat_id: i64,
    pub chat_uid: String,
    pub message_id: String,
    pub error: String,
    pub error_message: String,
}

impl From<longbridge::agent::ChatFinishedPayload> for ChatFinishedPayload {
    fn from(v: longbridge::agent::ChatFinishedPayload) -> Self {
        Self {
            chat_id: v.chat_id,
            chat_uid: v.chat_uid,
            message_id: v.message_id,
            error: v.error,
            error_message: v.error_message,
        }
    }
}

/// Payload of a `chat_title_updated` stream event — the server auto-generates
/// a short title for the conversation as a UI convenience. Can arrive before
/// *or* after `workflow_finished`; not tied to the run's outcome.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ChatTitleUpdatedPayload {
    pub chat_id: i64,
    pub chat_uid: String,
    pub source: String,
    pub title: String,
    pub updated_at: i64,
}

impl From<longbridge::agent::ChatTitleUpdatedPayload> for ChatTitleUpdatedPayload {
    fn from(v: longbridge::agent::ChatTitleUpdatedPayload) -> Self {
        Self {
            chat_id: v.chat_id,
            chat_uid: v.chat_uid,
            source: v.source,
            title: v.title,
            updated_at: v.updated_at,
        }
    }
}

/// One event observed while streaming
/// `AgentContext.conversation_streamed`/`continue_conversation_streamed` (or
/// the `Async` counterparts).
///
/// There's no existing precedent in this codebase for exposing a Rust
/// enum-with-payload to Python, so this flattens
/// `longbridge::agent::ConversationStreamEvent` into a single class: `kind` is
/// the discriminant (one of `"chat_started"`, `"workflow_started"`,
/// `"message"`, `"ping"`, `"chat_finished"`, `"workflow_finished"`,
/// `"chat_title_updated"`, `"other"`) and exactly one of
/// `chat_started`/`workflow_started`/`message`/`chat_finished`/
/// `workflow_finished`/`chat_title_updated`/`other` is set, matching `kind` —
/// except `"ping"`, a heartbeat with no payload, for which every payload field
/// is `None`. When `kind` is `"other"`, `other_event` additionally carries the
/// SSE envelope's `event` field (the event type name).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct ConversationStreamEvent {
    pub kind: String,
    pub chat_started: Option<ChatStartedPayload>,
    pub workflow_started: Option<WorkflowStartedPayload>,
    pub message: Option<MessagePayload>,
    pub chat_finished: Option<ChatFinishedPayload>,
    pub workflow_finished: Option<ConversationResponse>,
    pub chat_title_updated: Option<ChatTitleUpdatedPayload>,
    pub other_event: Option<String>,
    pub other: Option<JsonValue>,
}

impl From<longbridge::agent::ConversationStreamEvent> for ConversationStreamEvent {
    fn from(v: longbridge::agent::ConversationStreamEvent) -> Self {
        use longbridge::agent::ConversationStreamEvent as E;

        match v {
            E::ChatStarted(payload) => Self {
                kind: "chat_started".to_string(),
                chat_started: Some(payload.into()),
                workflow_started: None,
                message: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::WorkflowStarted(payload) => Self {
                kind: "workflow_started".to_string(),
                chat_started: None,
                workflow_started: Some(payload.into()),
                message: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::Message(payload) => Self {
                kind: "message".to_string(),
                chat_started: None,
                workflow_started: None,
                message: Some(payload.into()),
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::Ping => Self {
                kind: "ping".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::ChatFinished(payload) => Self {
                kind: "chat_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                chat_finished: Some(payload.into()),
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::WorkflowFinished(resp) => Self {
                kind: "workflow_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                chat_finished: None,
                workflow_finished: Some(resp.into()),
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            E::ChatTitleUpdated(payload) => Self {
                kind: "chat_title_updated".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: Some(payload.into()),
                other_event: None,
                other: None,
            },
            E::Other { event, data } => Self {
                kind: "other".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: Some(event),
                other: Some(JsonValue(data)),
            },
        }
    }
}
