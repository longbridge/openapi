use std::os::raw::c_char;

use longbridge::agent::{
    Agent, AgentError, AgentsResponse, ChatStartedPayload, ConversationResponse,
    ConversationStreamEvent, Interrupt, MessagePayload, Question, QuestionOption, Reference,
    Workspace, WorkspacesResponse,
};

use crate::{
    agent_context::enum_types::{CConversationStatus, CConversationStreamEventType},
    types::{CCow, CString, CVec, ToFFI},
};

/// A Workspace the current account belongs to
#[repr(C)]
pub struct CWorkspace {
    /// Workspace ID
    pub id: *const c_char,
    /// Workspace name
    pub name: *const c_char,
    /// Creation time, Unix timestamp in seconds
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    pub updated_at: i64,
}

#[derive(Debug)]
pub(crate) struct CWorkspaceOwned {
    id: CString,
    name: CString,
    created_at: i64,
    updated_at: i64,
}

impl From<Workspace> for CWorkspaceOwned {
    fn from(v: Workspace) -> Self {
        let Workspace {
            id,
            name,
            created_at,
            updated_at,
        } = v;
        Self {
            id: id.into(),
            name: name.into(),
            created_at,
            updated_at,
        }
    }
}

impl ToFFI for CWorkspaceOwned {
    type FFIType = CWorkspace;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWorkspaceOwned {
            id,
            name,
            created_at,
            updated_at,
        } = self;
        CWorkspace {
            id: id.to_ffi_type(),
            name: name.to_ffi_type(),
            created_at: *created_at,
            updated_at: *updated_at,
        }
    }
}

/// Response for `lb_agent_context_workspaces`
#[repr(C)]
pub struct CWorkspacesResponse {
    /// Workspaces the current account belongs to
    pub workspaces: *const CWorkspace,
    /// Number of workspaces
    pub num_workspaces: usize,
}

pub(crate) struct CWorkspacesResponseOwned {
    workspaces: CVec<CWorkspaceOwned>,
}

impl From<WorkspacesResponse> for CWorkspacesResponseOwned {
    fn from(v: WorkspacesResponse) -> Self {
        Self {
            workspaces: v.workspaces.into(),
        }
    }
}

impl ToFFI for CWorkspacesResponseOwned {
    type FFIType = CWorkspacesResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CWorkspacesResponse {
            workspaces: self.workspaces.to_ffi_type(),
            num_workspaces: self.workspaces.len(),
        }
    }
}

/// An Agent in a Workspace
#[repr(C)]
pub struct CAgent {
    /// Agent UID, used as the path parameter of
    /// `lb_agent_context_conversation`
    pub uid: *const c_char,
    /// Agent name
    pub name: *const c_char,
    /// Agent description
    pub description: *const c_char,
    /// Agent mode, e.g. `chat`
    pub mode: *const c_char,
    /// Icon URL
    pub icon: *const c_char,
    /// Whether published; only published Agents can start conversations
    pub is_published: bool,
    /// Publish time, Unix timestamp in seconds; 0 if unpublished
    pub published_at: i64,
    /// Creation time, Unix timestamp in seconds
    pub created_at: i64,
    /// Last updated time, Unix timestamp in seconds
    pub updated_at: i64,
}

#[derive(Debug)]
pub(crate) struct CAgentOwned {
    uid: CString,
    name: CString,
    description: CString,
    mode: CString,
    icon: CString,
    is_published: bool,
    published_at: i64,
    created_at: i64,
    updated_at: i64,
}

impl From<Agent> for CAgentOwned {
    fn from(v: Agent) -> Self {
        let Agent {
            uid,
            name,
            description,
            mode,
            icon,
            is_published,
            published_at,
            created_at,
            updated_at,
        } = v;
        Self {
            uid: uid.into(),
            name: name.into(),
            description: description.into(),
            mode: mode.into(),
            icon: icon.into(),
            is_published,
            published_at,
            created_at,
            updated_at,
        }
    }
}

impl ToFFI for CAgentOwned {
    type FFIType = CAgent;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAgentOwned {
            uid,
            name,
            description,
            mode,
            icon,
            is_published,
            published_at,
            created_at,
            updated_at,
        } = self;
        CAgent {
            uid: uid.to_ffi_type(),
            name: name.to_ffi_type(),
            description: description.to_ffi_type(),
            mode: mode.to_ffi_type(),
            icon: icon.to_ffi_type(),
            is_published: *is_published,
            published_at: *published_at,
            created_at: *created_at,
            updated_at: *updated_at,
        }
    }
}

/// Response for `lb_agent_context_agents`
#[repr(C)]
pub struct CAgentsResponse {
    /// Agent list
    pub agents: *const CAgent,
    /// Number of agents in the array
    pub num_agents: usize,
    /// Total number of matching Agents
    pub total: i32,
}

pub(crate) struct CAgentsResponseOwned {
    agents: CVec<CAgentOwned>,
    total: i32,
}

impl From<AgentsResponse> for CAgentsResponseOwned {
    fn from(v: AgentsResponse) -> Self {
        let AgentsResponse { agents, total } = v;
        Self {
            agents: agents.into(),
            total,
        }
    }
}

impl ToFFI for CAgentsResponseOwned {
    type FFIType = CAgentsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CAgentsResponse {
            agents: self.agents.to_ffi_type(),
            num_agents: self.agents.len(),
            total: self.total,
        }
    }
}

/// Options for `lb_agent_context_agents` (all fields can be null)
#[repr(C)]
pub struct CGetAgentsOptions {
    /// Page number, starts at 1 (can be null)
    pub page: *const i32,
    /// Page size (can be null)
    pub limit: *const i32,
    /// Fuzzy search by Agent name (can be null)
    pub name: *const c_char,
}

/// A source referenced by the answer
#[repr(C)]
pub struct CReference {
    /// Reference index
    pub index: i32,
    /// Reference title
    pub title: *const c_char,
    /// Reference URL
    pub url: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CReferenceOwned {
    index: i32,
    title: CString,
    url: CString,
}

impl From<Reference> for CReferenceOwned {
    fn from(v: Reference) -> Self {
        let Reference { index, title, url } = v;
        Self {
            index,
            title: title.into(),
            url: url.into(),
        }
    }
}

impl ToFFI for CReferenceOwned {
    type FFIType = CReference;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CReferenceOwned { index, title, url } = self;
        CReference {
            index: *index,
            title: title.to_ffi_type(),
            url: url.to_ffi_type(),
        }
    }
}

/// One option of a [`CQuestion`]
#[repr(C)]
pub struct CQuestionOption {
    /// Option text
    pub description: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CQuestionOptionOwned {
    description: CString,
}

impl From<QuestionOption> for CQuestionOptionOwned {
    fn from(v: QuestionOption) -> Self {
        Self {
            description: v.description.into(),
        }
    }
}

impl ToFFI for CQuestionOptionOwned {
    type FFIType = CQuestionOption;

    fn to_ffi_type(&self) -> Self::FFIType {
        CQuestionOption {
            description: self.description.to_ffi_type(),
        }
    }
}

/// One question the Agent needs you to answer
#[repr(C)]
pub struct CQuestion {
    /// Question text
    pub question: *const c_char,
    /// Options; empty means free-form answer
    pub options: *const CQuestionOption,
    /// Number of options
    pub num_options: usize,
    /// Whether multiple options may be selected
    pub multi_select: bool,
}

#[derive(Debug)]
pub(crate) struct CQuestionOwned {
    question: CString,
    options: CVec<CQuestionOptionOwned>,
    multi_select: bool,
}

impl From<Question> for CQuestionOwned {
    fn from(v: Question) -> Self {
        let Question {
            question,
            options,
            multi_select,
        } = v;
        Self {
            question: question.into(),
            options: options.into(),
            multi_select,
        }
    }
}

impl ToFFI for CQuestionOwned {
    type FFIType = CQuestion;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CQuestionOwned {
            question,
            options,
            multi_select,
        } = self;
        CQuestion {
            question: question.to_ffi_type(),
            options: options.to_ffi_type(),
            num_options: options.len(),
            multi_select: *multi_select,
        }
    }
}

/// Present when a conversation run is interrupted, waiting for
/// `lb_agent_context_continue_conversation`
#[repr(C)]
pub struct CInterrupt {
    /// ID of the node that triggered the interrupt
    pub node_id: *const c_char,
    /// Tool call ID of this inquiry; used as the answer key when continuing
    pub tool_call_id: *const c_char,
    /// Questions you need to answer
    pub questions: *const CQuestion,
    /// Number of questions
    pub num_questions: usize,
    /// ID of the paused message
    pub message_id: i64,
    /// ID of the owning conversation
    pub chat_id: i64,
}

#[derive(Debug)]
pub(crate) struct CInterruptOwned {
    node_id: CString,
    tool_call_id: CString,
    questions: CVec<CQuestionOwned>,
    message_id: i64,
    chat_id: i64,
}

impl From<Interrupt> for CInterruptOwned {
    fn from(v: Interrupt) -> Self {
        let Interrupt {
            node_id,
            tool_call_id,
            questions,
            message_id,
            chat_id,
        } = v;
        Self {
            node_id: node_id.into(),
            tool_call_id: tool_call_id.into(),
            questions: questions.into(),
            message_id,
            chat_id,
        }
    }
}

impl ToFFI for CInterruptOwned {
    type FFIType = CInterrupt;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CInterruptOwned {
            node_id,
            tool_call_id,
            questions,
            message_id,
            chat_id,
        } = self;
        CInterrupt {
            node_id: node_id.to_ffi_type(),
            tool_call_id: tool_call_id.to_ffi_type(),
            questions: questions.to_ffi_type(),
            num_questions: questions.len(),
            message_id: *message_id,
            chat_id: *chat_id,
        }
    }
}

/// Present when a conversation run failed
#[repr(C)]
pub struct CAgentError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CAgentErrorOwned {
    code: i32,
    message: CString,
}

impl From<AgentError> for CAgentErrorOwned {
    fn from(v: AgentError) -> Self {
        let AgentError { code, message } = v;
        Self {
            code,
            message: message.into(),
        }
    }
}

impl ToFFI for CAgentErrorOwned {
    type FFIType = CAgentError;

    fn to_ffi_type(&self) -> Self::FFIType {
        CAgentError {
            code: self.code,
            message: self.message.to_ffi_type(),
        }
    }
}

/// Response for `lb_agent_context_conversation`,
/// `lb_agent_context_continue_conversation`, and the final result of the
/// streamed counterparts
#[repr(C)]
pub struct CConversationResponse {
    /// Conversation identifier, used for follow-up questions and
    /// troubleshooting
    pub chat_uid: *const c_char,
    /// Message ID of this round
    pub message_id: *const c_char,
    /// Final run status
    pub status: CConversationStatus,
    /// Final answer text; valid when `status` is
    /// `ConversationStatusSucceeded`
    pub answer: *const c_char,
    /// Sources referenced by the answer
    pub references: *const CReference,
    /// Number of references
    pub num_references: usize,
    /// Run duration in seconds
    pub elapsed_time: f64,
    /// Present only when `status` is `ConversationStatusInterrupted` (can be
    /// null)
    pub interrupt: *const CInterrupt,
    /// Present only when the run failed (can be null)
    pub error: *const CAgentError,
}

pub(crate) struct CConversationResponseOwned {
    chat_uid: CString,
    message_id: CString,
    status: CConversationStatus,
    answer: CString,
    references: CVec<CReferenceOwned>,
    elapsed_time: f64,
    interrupt: Option<CCow<CInterruptOwned>>,
    error: Option<CCow<CAgentErrorOwned>>,
}

impl From<ConversationResponse> for CConversationResponseOwned {
    fn from(v: ConversationResponse) -> Self {
        let ConversationResponse {
            chat_uid,
            message_id,
            status,
            answer,
            references,
            elapsed_time,
            interrupt,
            error,
        } = v;
        Self {
            chat_uid: chat_uid.into(),
            message_id: message_id.into(),
            status: status.into(),
            answer: answer.into(),
            // `references` is `Option<Vec<Reference>>`; there's no FFI-level
            // distinction between "absent" and "empty" here, both surface as
            // `num_references == 0`.
            references: references.unwrap_or_default().into(),
            elapsed_time,
            interrupt: interrupt.map(CCow::new),
            error: error.map(CCow::new),
        }
    }
}

impl ToFFI for CConversationResponseOwned {
    type FFIType = CConversationResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CConversationResponseOwned {
            chat_uid,
            message_id,
            status,
            answer,
            references,
            elapsed_time,
            interrupt,
            error,
        } = self;
        CConversationResponse {
            chat_uid: chat_uid.to_ffi_type(),
            message_id: message_id.to_ffi_type(),
            status: *status,
            answer: answer.to_ffi_type(),
            references: references.to_ffi_type(),
            num_references: references.len(),
            elapsed_time: *elapsed_time,
            interrupt: interrupt
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            error: error
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
        }
    }
}

/// Payload of a `ChatStarted` conversation stream event
#[repr(C)]
pub struct CChatStartedPayload {
    /// Conversation identifier
    pub chat_uid: *const c_char,
    /// Message ID of this round
    pub message_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CChatStartedPayloadOwned {
    chat_uid: CString,
    message_id: CString,
}

impl From<ChatStartedPayload> for CChatStartedPayloadOwned {
    fn from(v: ChatStartedPayload) -> Self {
        let ChatStartedPayload {
            chat_uid,
            message_id,
        } = v;
        Self {
            chat_uid: chat_uid.into(),
            message_id: message_id.into(),
        }
    }
}

impl ToFFI for CChatStartedPayloadOwned {
    type FFIType = CChatStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        CChatStartedPayload {
            chat_uid: self.chat_uid.to_ffi_type(),
            message_id: self.message_id.to_ffi_type(),
        }
    }
}

/// Payload of a `Message` conversation stream event
#[repr(C)]
pub struct CMessagePayload {
    /// Incremental answer text
    pub text: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CMessagePayloadOwned {
    text: CString,
}

impl From<MessagePayload> for CMessagePayloadOwned {
    fn from(v: MessagePayload) -> Self {
        Self {
            text: v.text.into(),
        }
    }
}

impl ToFFI for CMessagePayloadOwned {
    type FFIType = CMessagePayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        CMessagePayload {
            text: self.text.to_ffi_type(),
        }
    }
}

/// One event observed while streaming `lb_agent_context_conversation_streamed`
/// or `lb_agent_context_continue_conversation_streamed`.
///
/// This is a tagged union: `kind` tells you which of `chat_started`/
/// `message`/`workflow_finished`/`other_json` is non-null; the other three are
/// always null.
#[repr(C)]
pub struct CConversationStreamEvent {
    /// Discriminant, tells you which field below is populated
    pub kind: CConversationStreamEventType,
    /// Non-null when `kind` is `ChatStarted`
    pub chat_started: *const CChatStartedPayload,
    /// Non-null when `kind` is `Message`
    pub message: *const CMessagePayload,
    /// Non-null when `kind` is `WorkflowFinished` — this is always the last
    /// event of a stream
    pub workflow_finished: *const CConversationResponse,
    /// Non-null when `kind` is `Other`; the SSE envelope's `event` field (the
    /// event type name), e.g. `"workflow_started"`, `"ping"`,
    /// `"chat_finished"`, `"chat_title_updated"` (observed against the real
    /// API; not documented)
    pub other_event: *const c_char,
    /// Non-null when `kind` is `Other`; raw JSON of an event type not
    /// recognized by this SDK version
    pub other_json: *const c_char,
}

pub(crate) enum CConversationStreamEventOwned {
    ChatStarted(CCow<CChatStartedPayloadOwned>),
    Message(CCow<CMessagePayloadOwned>),
    WorkflowFinished(CCow<CConversationResponseOwned>),
    Other { event: CString, json: CString },
}

impl From<ConversationStreamEvent> for CConversationStreamEventOwned {
    fn from(v: ConversationStreamEvent) -> Self {
        match v {
            ConversationStreamEvent::ChatStarted(payload) => Self::ChatStarted(CCow::new(payload)),
            ConversationStreamEvent::Message(payload) => Self::Message(CCow::new(payload)),
            ConversationStreamEvent::WorkflowFinished(resp) => {
                Self::WorkflowFinished(CCow::new(resp))
            }
            // `Other` carries an arbitrary `serde_json::Value` (events from
            // future SDK versions we don't recognize yet) — re-serialize it
            // to a JSON string so C callers can still inspect it, alongside
            // the discriminating `event` type name.
            ConversationStreamEvent::Other { event, data } => Self::Other {
                event: event.into(),
                json: serde_json::to_string(&data).unwrap_or_default().into(),
            },
        }
    }
}

impl ToFFI for CConversationStreamEventOwned {
    type FFIType = CConversationStreamEvent;

    fn to_ffi_type(&self) -> Self::FFIType {
        match self {
            Self::ChatStarted(payload) => CConversationStreamEvent {
                kind: CConversationStreamEventType::ChatStarted,
                chat_started: payload.to_ffi_type(),
                message: std::ptr::null(),
                workflow_finished: std::ptr::null(),
                other_event: std::ptr::null(),
                other_json: std::ptr::null(),
            },
            Self::Message(payload) => CConversationStreamEvent {
                kind: CConversationStreamEventType::Message,
                chat_started: std::ptr::null(),
                message: payload.to_ffi_type(),
                workflow_finished: std::ptr::null(),
                other_event: std::ptr::null(),
                other_json: std::ptr::null(),
            },
            Self::WorkflowFinished(resp) => CConversationStreamEvent {
                kind: CConversationStreamEventType::WorkflowFinished,
                chat_started: std::ptr::null(),
                message: std::ptr::null(),
                workflow_finished: resp.to_ffi_type(),
                other_event: std::ptr::null(),
                other_json: std::ptr::null(),
            },
            Self::Other { event, json } => CConversationStreamEvent {
                kind: CConversationStreamEventType::Other,
                chat_started: std::ptr::null(),
                message: std::ptr::null(),
                workflow_finished: std::ptr::null(),
                other_event: event.to_ffi_type(),
                other_json: json.to_ffi_type(),
            },
        }
    }
}

/// One answer to a [`CInterrupt`] question, used as an entry of
/// [`CAnswersByToolCallEntry::answers`]
#[repr(C)]
pub struct CAnswerQuestion {
    /// Question text, must match `CQuestion::question` verbatim
    pub question: *const c_char,
    /// Your answer text
    pub answer: *const c_char,
}

/// Answers for one `tool_call_id`, used as an entry of the `answers` array of
/// `lb_agent_context_continue_conversation`/
/// `lb_agent_context_continue_conversation_streamed`.
///
/// The Rust core's `AnswersByToolCall` is a
/// `HashMap<String, HashMap<String, String>>` keyed by `tool_call_id`, then by
/// question text. Since C has no native map type, it's flattened into an
/// array of `(tool_call_id, [(question, answer)])` entries — this array of
/// `CAnswersByToolCallEntry` mirrors the outer map, and each entry's
/// `answers` array (of `CAnswerQuestion`) mirrors the inner map.
#[repr(C)]
pub struct CAnswersByToolCallEntry {
    /// Tool call ID, see [`CInterrupt::tool_call_id`]
    pub tool_call_id: *const c_char,
    /// Answers to the questions raised for this tool call
    pub answers: *const CAnswerQuestion,
    /// Number of answers
    pub num_answers: usize,
}
