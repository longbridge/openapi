use longbridge::agent::types as lb;

/// A Workspace the current account belongs to
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Creation time, Unix timestamp in seconds
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    pub updated_at: i64,
}
impl From<lb::Workspace> for Workspace {
    fn from(v: lb::Workspace) -> Self {
        Self {
            id: v.id,
            name: v.name,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

/// Response for `AgentContext.workspaces`
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct WorkspacesResponse {
    /// Workspaces the current account belongs to
    pub workspaces: Vec<Workspace>,
}
impl From<lb::WorkspacesResponse> for WorkspacesResponse {
    fn from(v: lb::WorkspacesResponse) -> Self {
        Self {
            workspaces: v.workspaces.into_iter().map(Into::into).collect(),
        }
    }
}

/// An Agent in a Workspace
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Agent {
    /// Agent UID, used as the path parameter of `AgentContext.conversation`
    pub uid: String,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: String,
    /// Agent mode, e.g. `chat`
    pub mode: String,
    /// Icon URL
    pub icon: String,
    /// Whether published; only published Agents can start conversations
    pub is_published: bool,
    /// Publish time, Unix timestamp in seconds; 0 if unpublished
    pub published_at: i64,
    /// Creation time, Unix timestamp in seconds
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    pub updated_at: i64,
}
impl From<lb::Agent> for Agent {
    fn from(v: lb::Agent) -> Self {
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

/// Response for `AgentContext.agents`
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AgentsResponse {
    /// Agent list
    pub agents: Vec<Agent>,
    /// Total number of matching Agents
    pub total: i32,
}
impl From<lb::AgentsResponse> for AgentsResponse {
    fn from(v: lb::AgentsResponse) -> Self {
        Self {
            agents: v.agents.into_iter().map(Into::into).collect(),
            total: v.total,
        }
    }
}

/// Final run status of a conversation
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum ConversationStatus {
    /// The run completed successfully
    Succeeded,
    /// The run is paused, waiting for `AgentContext.continueConversation`
    Interrupted,
    /// The run failed
    Failed,
    /// The run was stopped
    Stopped,
}
impl From<lb::ConversationStatus> for ConversationStatus {
    fn from(v: lb::ConversationStatus) -> Self {
        match v {
            lb::ConversationStatus::Succeeded => ConversationStatus::Succeeded,
            lb::ConversationStatus::Interrupted => ConversationStatus::Interrupted,
            lb::ConversationStatus::Failed => ConversationStatus::Failed,
            lb::ConversationStatus::Stopped => ConversationStatus::Stopped,
        }
    }
}

/// A source referenced by the answer
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Reference {
    /// Reference index
    pub index: i32,
    /// Reference title
    pub title: String,
    /// Reference URL
    pub url: String,
}
impl From<lb::Reference> for Reference {
    fn from(v: lb::Reference) -> Self {
        Self {
            index: v.index,
            title: v.title,
            url: v.url,
        }
    }
}

/// One option of a `Question`
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct QuestionOption {
    /// Option text
    pub description: String,
}
impl From<lb::QuestionOption> for QuestionOption {
    fn from(v: lb::QuestionOption) -> Self {
        Self {
            description: v.description,
        }
    }
}

/// One question the Agent needs you to answer
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Question {
    /// Question text
    pub question: String,
    /// Options; empty means free-form answer
    pub options: Vec<QuestionOption>,
    /// Whether multiple options may be selected
    pub multi_select: bool,
}
impl From<lb::Question> for Question {
    fn from(v: lb::Question) -> Self {
        Self {
            question: v.question,
            options: v.options.into_iter().map(Into::into).collect(),
            multi_select: v.multi_select,
        }
    }
}

/// Present when a conversation run is interrupted, waiting for
/// `AgentContext.continueConversation`
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct Interrupt {
    /// ID of the node that triggered the interrupt
    pub node_id: String,
    /// Tool call ID of this inquiry; used as the answer key when continuing
    pub tool_call_id: String,
    /// Questions you need to answer
    pub questions: Vec<Question>,
    /// ID of the paused message
    pub message_id: i64,
    /// ID of the owning conversation
    pub chat_id: i64,
}
impl From<lb::Interrupt> for Interrupt {
    fn from(v: lb::Interrupt) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AgentError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
}
impl From<lb::AgentError> for AgentError {
    fn from(v: lb::AgentError) -> Self {
        Self {
            code: v.code,
            message: v.message,
        }
    }
}

/// Response for `AgentContext.conversation`,
/// `AgentContext.continueConversation`, and the final result of the streamed
/// counterparts
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConversationResponse {
    /// Conversation identifier, used for follow-up questions and
    /// troubleshooting
    pub chat_uid: String,
    /// Message ID of this round
    pub message_id: String,
    /// Final run status
    pub status: ConversationStatus,
    /// Final answer text; valid when `status` is `succeeded`
    pub answer: String,
    /// Sources referenced by the answer
    pub references: Option<Vec<Reference>>,
    /// Run duration in seconds
    pub elapsed_time: f64,
    /// Present only when `status` is `interrupted`
    pub interrupt: Option<Interrupt>,
    /// Present only when the run failed
    pub error: Option<AgentError>,
}
impl From<lb::ConversationResponse> for ConversationResponse {
    fn from(v: lb::ConversationResponse) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ChatStartedPayload {
    /// Conversation identifier
    pub chat_uid: String,
    /// Message ID of this round
    pub message_id: String,
}
impl From<lb::ChatStartedPayload> for ChatStartedPayload {
    fn from(v: lb::ChatStartedPayload) -> Self {
        Self {
            chat_uid: v.chat_uid,
            message_id: v.message_id,
        }
    }
}

/// Payload of a `message` stream event
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MessagePayload {
    /// Incremental answer text
    pub text: String,
}
impl From<lb::MessagePayload> for MessagePayload {
    fn from(v: lb::MessagePayload) -> Self {
        Self { text: v.text }
    }
}

/// One event observed while streaming `AgentContext.conversationStreamed` or
/// `AgentContext.continueConversationStreamed`.
///
/// Design note: the Rust core models this as an enum with a per-variant
/// payload (`longbridge::agent::ConversationStreamEvent`), but napi-rs has no
/// ergonomic equivalent of a Rust/Serde "enum with data" for a plain
/// `#[napi(object)]` value, and there's no existing precedent for reifying one
/// as a single JS value in this codebase (the closest analogue,
/// `trade::PushEvent`, is dispatched to separate per-variant JS callbacks
/// instead). We instead mirror the common "discriminant + optional per-kind
/// fields" shape used for tagged unions in plain JS/JSON: `kind` is one of
/// `"chat_started" | "message" | "workflow_finished" | "other"`, and exactly
/// one of `chatStarted` / `message` / `workflowFinished` / `other` is set,
/// matching `kind`. When `kind` is `"other"`, `otherEvent` additionally
/// carries the SSE envelope's `event` field (the event type name).
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConversationStreamEvent {
    /// Discriminant: one of `"chat_started"`, `"message"`,
    /// `"workflow_finished"`, or `"other"`
    pub kind: String,
    /// Set when `kind` is `"chat_started"`
    pub chat_started: Option<ChatStartedPayload>,
    /// Set when `kind` is `"message"`
    pub message: Option<MessagePayload>,
    /// Set when `kind` is `"workflow_finished"` — the last event of a stream
    pub workflow_finished: Option<ConversationResponse>,
    /// Set when `kind` is `"other"` — the SSE envelope's `event` field (the
    /// event type name), e.g. `"workflow_started"`, `"ping"`,
    /// `"chat_finished"`, `"chat_title_updated"` (observed against the real
    /// API; not documented)
    pub other_event: Option<String>,
    /// Set when `kind` is `"other"` — raw JSON of an event type not
    /// recognized by this SDK version
    pub other: Option<serde_json::Value>,
}
impl From<lb::ConversationStreamEvent> for ConversationStreamEvent {
    fn from(v: lb::ConversationStreamEvent) -> Self {
        match v {
            lb::ConversationStreamEvent::ChatStarted(payload) => Self {
                kind: "chat_started".to_string(),
                chat_started: Some(payload.into()),
                message: None,
                workflow_finished: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::Message(payload) => Self {
                kind: "message".to_string(),
                chat_started: None,
                message: Some(payload.into()),
                workflow_finished: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::WorkflowFinished(resp) => Self {
                kind: "workflow_finished".to_string(),
                chat_started: None,
                message: None,
                workflow_finished: Some(resp.into()),
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::Other { event, data } => Self {
                kind: "other".to_string(),
                chat_started: None,
                message: None,
                workflow_finished: None,
                other_event: Some(event),
                other: Some(data),
            },
        }
    }
}
