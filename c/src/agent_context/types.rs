use std::os::raw::c_char;

use longbridge::agent::{
    Agent, AgentError, AgentToolFinishedPayload, AgentToolProgressPayload, AgentToolStartedPayload,
    AgentsResponse, ChatFinishedPayload, ChatStartedPayload, ChatTitleUpdatedPayload,
    ContextCompressFinishedPayload, ContextCompressStartedPayload, ConversationResponse,
    ConversationStreamEvent, Interrupt, MessagePayload, NodeToolUseFinishedPayload,
    NodeToolUseOutputs, NodeToolUseStartedPayload, PlanChangedPayload, QueryMaskedPayload,
    Question, QuestionOption, Reference, SubagentFinishedPayload, SubagentOutputs,
    SubagentProgressPayload, SubagentStartedPayload, ThinkingFinishedPayload,
    ThinkingStartedPayload, WorkflowStartedInputs, WorkflowStartedPayload, Workspace,
    WorkspacesResponse,
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

/// Payload of a `Message` conversation stream event — an incremental text
/// chunk. This is the highest-frequency event; concatenate `text` fragments
/// in arrival order.
#[repr(C)]
pub struct CMessagePayload {
    /// Incremental text fragment
    pub text: *const c_char,
    /// `answer` — final answer text; `think` — reasoning process; `process`
    /// — stage progress description
    pub message_type: *const c_char,
    /// Identifier of the stream segment this fragment belongs to. Fragments
    /// with the same `key` form one continuous block — group by `key` when
    /// rendering
    pub key: *const c_char,
    /// Time this segment started, Unix timestamp in seconds
    pub started_at: i64,
    /// Stage identifier; only present when `message_type` is `"process"`
    pub stage: *const c_char,
    /// Stage title while running; only present when `message_type` is
    /// `"process"`
    pub stage_title: *const c_char,
    /// Stage title after it finishes; only present when `message_type` is
    /// `"process"`
    pub stage_finished_title: *const c_char,
    /// Extra payload attached to the fragment, as a JSON string; empty when
    /// absent
    pub outputs_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CMessagePayloadOwned {
    text: CString,
    message_type: CString,
    key: CString,
    started_at: i64,
    stage: CString,
    stage_title: CString,
    stage_finished_title: CString,
    outputs_json: CString,
}

impl From<MessagePayload> for CMessagePayloadOwned {
    fn from(v: MessagePayload) -> Self {
        let MessagePayload {
            text,
            message_type,
            key,
            started_at,
            stage,
            stage_title,
            stage_finished_title,
            outputs,
        } = v;
        Self {
            text: text.into(),
            message_type: message_type.into(),
            key: key.into(),
            started_at,
            stage: stage.into(),
            stage_title: stage_title.into(),
            stage_finished_title: stage_finished_title.into(),
            outputs_json: outputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CMessagePayloadOwned {
    type FFIType = CMessagePayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CMessagePayloadOwned {
            text,
            message_type,
            key,
            started_at,
            stage,
            stage_title,
            stage_finished_title,
            outputs_json,
        } = self;
        CMessagePayload {
            text: text.to_ffi_type(),
            message_type: message_type.to_ffi_type(),
            key: key.to_ffi_type(),
            started_at: *started_at,
            stage: stage.to_ffi_type(),
            stage_title: stage_title.to_ffi_type(),
            stage_finished_title: stage_finished_title.to_ffi_type(),
            outputs_json: outputs_json.to_ffi_type(),
        }
    }
}

/// `inputs` of a `WorkflowStarted` conversation stream event
#[repr(C)]
pub struct CWorkflowStartedInputs {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: *const c_char,
    /// Message ID of this round
    pub message_id: *const c_char,
    /// The question that was asked
    pub query: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CWorkflowStartedInputsOwned {
    chat_id: i64,
    chat_uid: CString,
    message_id: CString,
    query: CString,
}

impl From<WorkflowStartedInputs> for CWorkflowStartedInputsOwned {
    fn from(v: WorkflowStartedInputs) -> Self {
        let WorkflowStartedInputs {
            chat_id,
            chat_uid,
            message_id,
            query,
        } = v;
        Self {
            chat_id,
            chat_uid: chat_uid.into(),
            message_id: message_id.into(),
            query: query.into(),
        }
    }
}

impl ToFFI for CWorkflowStartedInputsOwned {
    type FFIType = CWorkflowStartedInputs;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWorkflowStartedInputsOwned {
            chat_id,
            chat_uid,
            message_id,
            query,
        } = self;
        CWorkflowStartedInputs {
            chat_id: *chat_id,
            chat_uid: chat_uid.to_ffi_type(),
            message_id: message_id.to_ffi_type(),
            query: query.to_ffi_type(),
        }
    }
}

/// Payload of a `WorkflowStarted` conversation stream event, observed right
/// after `ChatStarted` on every run seen so far
#[repr(C)]
pub struct CWorkflowStartedPayload {
    /// Whether this run's answer was served from a cache
    pub hit_cache: bool,
    /// Echoes the run's inputs
    pub inputs: *const CWorkflowStartedInputs,
    /// Unix timestamp in seconds
    pub started_at: i64,
    /// Internal workflow run ID
    pub workflow_id: i64,
}

#[derive(Debug)]
pub(crate) struct CWorkflowStartedPayloadOwned {
    hit_cache: bool,
    inputs: CCow<CWorkflowStartedInputsOwned>,
    started_at: i64,
    workflow_id: i64,
}

impl From<WorkflowStartedPayload> for CWorkflowStartedPayloadOwned {
    fn from(v: WorkflowStartedPayload) -> Self {
        let WorkflowStartedPayload {
            hit_cache,
            inputs,
            started_at,
            workflow_id,
        } = v;
        Self {
            hit_cache,
            inputs: CCow::new(inputs),
            started_at,
            workflow_id,
        }
    }
}

impl ToFFI for CWorkflowStartedPayloadOwned {
    type FFIType = CWorkflowStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CWorkflowStartedPayloadOwned {
            hit_cache,
            inputs,
            started_at,
            workflow_id,
        } = self;
        CWorkflowStartedPayload {
            hit_cache: *hit_cache,
            inputs: inputs.to_ffi_type(),
            started_at: *started_at,
            workflow_id: *workflow_id,
        }
    }
}

/// Payload of a `ThinkingStarted` conversation stream event — the Agent has
/// entered the reasoning phase (analyzing the question, planning tool
/// calls). Between this and `ThinkingFinished`, `Message` events with
/// `message_type == "think"` and tool-call events may arrive.
#[repr(C)]
pub struct CThinkingStartedPayload {
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}

#[derive(Debug)]
pub(crate) struct CThinkingStartedPayloadOwned {
    started_at: i64,
}

impl From<ThinkingStartedPayload> for CThinkingStartedPayloadOwned {
    fn from(v: ThinkingStartedPayload) -> Self {
        let ThinkingStartedPayload { started_at } = v;
        Self { started_at }
    }
}

impl ToFFI for CThinkingStartedPayloadOwned {
    type FFIType = CThinkingStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        CThinkingStartedPayload {
            started_at: self.started_at,
        }
    }
}

/// Payload of a `ThinkingFinished` conversation stream event — the
/// reasoning phase is over; answer text (`Message` with `message_type ==
/// "answer"`) follows.
#[repr(C)]
pub struct CThinkingFinishedPayload {
    /// Finish time, Unix timestamp in seconds
    pub finished_at: i64,
    /// Reasoning duration in seconds
    pub elapsed_time: i32,
}

#[derive(Debug)]
pub(crate) struct CThinkingFinishedPayloadOwned {
    finished_at: i64,
    elapsed_time: i32,
}

impl From<ThinkingFinishedPayload> for CThinkingFinishedPayloadOwned {
    fn from(v: ThinkingFinishedPayload) -> Self {
        let ThinkingFinishedPayload {
            finished_at,
            elapsed_time,
        } = v;
        Self {
            finished_at,
            elapsed_time,
        }
    }
}

impl ToFFI for CThinkingFinishedPayloadOwned {
    type FFIType = CThinkingFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        CThinkingFinishedPayload {
            finished_at: self.finished_at,
            elapsed_time: self.elapsed_time,
        }
    }
}

/// Payload of a `NodeToolUseStarted` conversation stream event — an
/// ordinary tool call has started. Match it to its `NodeToolUseFinished`
/// counterpart by `tool_use_id`.
#[repr(C)]
pub struct CNodeToolUseStartedPayload {
    /// Unique ID of this call; matches the finished event
    pub tool_use_id: *const c_char,
    /// Localized display name of the tool
    pub tool_name: *const c_char,
    /// Locale-stable tool identifier; use this for logic keyed on the tool
    /// kind
    pub tool_func_name: *const c_char,
    /// Call arguments as a JSON string
    pub tool_args: *const c_char,
    /// Progress text suitable for direct display, e.g. `"Searching the
    /// web…"`
    pub tips: *const c_char,
    /// Short tags accompanying `tips`; may be empty
    pub tip_chips: *const *const c_char,
    /// Number of tags in `tip_chips`
    pub num_tip_chips: usize,
    /// Round number. Calls in the same round (same `iteration`) run in
    /// parallel
    pub iteration: i32,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}

#[derive(Debug)]
pub(crate) struct CNodeToolUseStartedPayloadOwned {
    tool_use_id: CString,
    tool_name: CString,
    tool_func_name: CString,
    tool_args: CString,
    tips: CString,
    tip_chips: CVec<CString>,
    iteration: i32,
    started_at: i64,
}

impl From<NodeToolUseStartedPayload> for CNodeToolUseStartedPayloadOwned {
    fn from(v: NodeToolUseStartedPayload) -> Self {
        let NodeToolUseStartedPayload {
            tool_use_id,
            tool_name,
            tool_func_name,
            tool_args,
            tips,
            tip_chips,
            iteration,
            started_at,
        } = v;
        Self {
            tool_use_id: tool_use_id.into(),
            tool_name: tool_name.into(),
            tool_func_name: tool_func_name.into(),
            tool_args: tool_args.into(),
            tips: tips.into(),
            tip_chips: tip_chips.into(),
            iteration,
            started_at,
        }
    }
}

impl ToFFI for CNodeToolUseStartedPayloadOwned {
    type FFIType = CNodeToolUseStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CNodeToolUseStartedPayloadOwned {
            tool_use_id,
            tool_name,
            tool_func_name,
            tool_args,
            tips,
            tip_chips,
            iteration,
            started_at,
        } = self;
        CNodeToolUseStartedPayload {
            tool_use_id: tool_use_id.to_ffi_type(),
            tool_name: tool_name.to_ffi_type(),
            tool_func_name: tool_func_name.to_ffi_type(),
            tool_args: tool_args.to_ffi_type(),
            tips: tips.to_ffi_type(),
            tip_chips: tip_chips.to_ffi_type(),
            num_tip_chips: tip_chips.len(),
            iteration: *iteration,
            started_at: *started_at,
        }
    }
}

/// `outputs` of a `NodeToolUseFinished` conversation stream event — only
/// carries fields meant for display
#[repr(C)]
pub struct CNodeToolUseOutputs {
    /// Sources referenced by the tool result
    pub references: *const CReference,
    /// Number of references
    pub num_references: usize,
    /// Domains of the referenced sources
    pub reference_domains: *const *const c_char,
    /// Number of reference domains
    pub num_reference_domains: usize,
    /// The query the tool executed; empty when absent
    pub query: *const c_char,
    /// Raw response text of the tool; empty when absent
    pub text: *const c_char,
    /// Parsed request arguments, as a JSON string; empty when absent
    pub tool_args_json: *const c_char,
    /// Structured result, as a JSON string; present only for selected
    /// tools, empty when absent
    pub data_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CNodeToolUseOutputsOwned {
    references: CVec<CReferenceOwned>,
    reference_domains: CVec<CString>,
    query: CString,
    text: CString,
    tool_args_json: CString,
    data_json: CString,
}

impl From<NodeToolUseOutputs> for CNodeToolUseOutputsOwned {
    fn from(v: NodeToolUseOutputs) -> Self {
        let NodeToolUseOutputs {
            references,
            reference_domains,
            query,
            text,
            tool_args,
            data,
        } = v;
        Self {
            references: references.unwrap_or_default().into(),
            reference_domains: reference_domains.unwrap_or_default().into(),
            query: query.unwrap_or_default().into(),
            text: text.unwrap_or_default().into(),
            tool_args_json: tool_args
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
            data_json: data
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CNodeToolUseOutputsOwned {
    type FFIType = CNodeToolUseOutputs;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CNodeToolUseOutputsOwned {
            references,
            reference_domains,
            query,
            text,
            tool_args_json,
            data_json,
        } = self;
        CNodeToolUseOutputs {
            references: references.to_ffi_type(),
            num_references: references.len(),
            reference_domains: reference_domains.to_ffi_type(),
            num_reference_domains: reference_domains.len(),
            query: query.to_ffi_type(),
            text: text.to_ffi_type(),
            tool_args_json: tool_args_json.to_ffi_type(),
            data_json: data_json.to_ffi_type(),
        }
    }
}

/// Payload of a `NodeToolUseFinished` conversation stream event — the tool
/// call has ended.
#[repr(C)]
pub struct CNodeToolUseFinishedPayload {
    /// Matches the `tool_use_id` of the started event
    pub tool_use_id: *const c_char,
    /// `succeeded` / `failed`
    pub status: *const c_char,
    /// Error description on failure
    pub error: *const c_char,
    /// Call duration in seconds
    pub elapsed_time: f64,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Localized display name
    pub tool_name: *const c_char,
    /// Locale-stable tool identifier
    pub tool_func_name: *const c_char,
    /// Call arguments as a JSON string
    pub tool_args: *const c_char,
    /// Tool category
    pub tool_type: *const c_char,
    /// Progress text
    pub tips: *const c_char,
    /// Short tags; may be empty
    pub tip_chips: *const *const c_char,
    /// Number of tags in `tip_chips`
    pub num_tip_chips: usize,
    /// Round number
    pub iteration: i32,
    /// `true` if the call happened during the thinking phase
    pub is_thinking: bool,
    /// Filtered call results, for display
    pub outputs: *const CNodeToolUseOutputs,
}

#[derive(Debug)]
pub(crate) struct CNodeToolUseFinishedPayloadOwned {
    tool_use_id: CString,
    status: CString,
    error: CString,
    elapsed_time: f64,
    started_at: i64,
    tool_name: CString,
    tool_func_name: CString,
    tool_args: CString,
    tool_type: CString,
    tips: CString,
    tip_chips: CVec<CString>,
    iteration: i32,
    is_thinking: bool,
    outputs: CCow<CNodeToolUseOutputsOwned>,
}

impl From<NodeToolUseFinishedPayload> for CNodeToolUseFinishedPayloadOwned {
    fn from(v: NodeToolUseFinishedPayload) -> Self {
        let NodeToolUseFinishedPayload {
            tool_use_id,
            status,
            error,
            elapsed_time,
            started_at,
            tool_name,
            tool_func_name,
            tool_args,
            tool_type,
            tips,
            tip_chips,
            iteration,
            is_thinking,
            outputs,
        } = v;
        Self {
            tool_use_id: tool_use_id.into(),
            status: status.into(),
            error: error.into(),
            elapsed_time,
            started_at,
            tool_name: tool_name.into(),
            tool_func_name: tool_func_name.into(),
            tool_args: tool_args.into(),
            tool_type: tool_type.into(),
            tips: tips.into(),
            tip_chips: tip_chips.into(),
            iteration,
            is_thinking,
            outputs: CCow::new(outputs),
        }
    }
}

impl ToFFI for CNodeToolUseFinishedPayloadOwned {
    type FFIType = CNodeToolUseFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CNodeToolUseFinishedPayloadOwned {
            tool_use_id,
            status,
            error,
            elapsed_time,
            started_at,
            tool_name,
            tool_func_name,
            tool_args,
            tool_type,
            tips,
            tip_chips,
            iteration,
            is_thinking,
            outputs,
        } = self;
        CNodeToolUseFinishedPayload {
            tool_use_id: tool_use_id.to_ffi_type(),
            status: status.to_ffi_type(),
            error: error.to_ffi_type(),
            elapsed_time: *elapsed_time,
            started_at: *started_at,
            tool_name: tool_name.to_ffi_type(),
            tool_func_name: tool_func_name.to_ffi_type(),
            tool_args: tool_args.to_ffi_type(),
            tool_type: tool_type.to_ffi_type(),
            tips: tips.to_ffi_type(),
            tip_chips: tip_chips.to_ffi_type(),
            num_tip_chips: tip_chips.len(),
            iteration: *iteration,
            is_thinking: *is_thinking,
            outputs: outputs.to_ffi_type(),
        }
    }
}

/// Payload of a `SubagentStarted` conversation stream event. When the Agent
/// spawns a subagent to work on a sub-task, the subagent's lifecycle is
/// reported with this dedicated event family instead of `NodeToolUse*`.
#[repr(C)]
pub struct CSubagentStartedPayload {
    /// ID of the node that spawned the subagent
    pub node_id: *const c_char,
    /// Unique ID of this spawn; matches the finished event
    pub tool_use_id: *const c_char,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Goal assigned to the subagent
    pub goal: *const c_char,
    /// Full task prompt given to the subagent
    pub prompt: *const c_char,
    /// Subagent identifier; may be empty
    pub subagent_id: *const c_char,
    /// Tools granted to the subagent, as a JSON array string; empty when
    /// absent
    pub tools_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSubagentStartedPayloadOwned {
    node_id: CString,
    tool_use_id: CString,
    started_at: i64,
    goal: CString,
    prompt: CString,
    subagent_id: CString,
    tools_json: CString,
}

impl From<SubagentStartedPayload> for CSubagentStartedPayloadOwned {
    fn from(v: SubagentStartedPayload) -> Self {
        let SubagentStartedPayload {
            node_id,
            tool_use_id,
            started_at,
            goal,
            prompt,
            subagent_id,
            tools,
        } = v;
        Self {
            node_id: node_id.into(),
            tool_use_id: tool_use_id.into(),
            started_at,
            goal: goal.into(),
            prompt: prompt.into(),
            subagent_id: subagent_id.into(),
            tools_json: serde_json::to_string(&tools).unwrap_or_default().into(),
        }
    }
}

impl ToFFI for CSubagentStartedPayloadOwned {
    type FFIType = CSubagentStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSubagentStartedPayloadOwned {
            node_id,
            tool_use_id,
            started_at,
            goal,
            prompt,
            subagent_id,
            tools_json,
        } = self;
        CSubagentStartedPayload {
            node_id: node_id.to_ffi_type(),
            tool_use_id: tool_use_id.to_ffi_type(),
            started_at: *started_at,
            goal: goal.to_ffi_type(),
            prompt: prompt.to_ffi_type(),
            subagent_id: subagent_id.to_ffi_type(),
            tools_json: tools_json.to_ffi_type(),
        }
    }
}

/// Payload of a `SubagentProgress` conversation stream event, emitted every
/// time the subagent calls one of its own tools. Use it to render a live
/// timeline inside the subagent card.
#[repr(C)]
pub struct CSubagentProgressPayload {
    /// ID of the node that spawned the subagent
    pub node_id: *const c_char,
    /// `tool_use_id` of the owning `SubagentStarted` event
    pub parent_tool_call_id: *const c_char,
    /// Name of the tool the subagent called
    pub subagent_tool_name: *const c_char,
    /// Arguments of that call, as a JSON string
    pub subagent_tool_args: *const c_char,
    /// Status of that call: `running` / `succeeded` / `failed`
    pub subagent_status: *const c_char,
    /// Duration of that call in milliseconds
    pub subagent_duration_ms: i64,
    /// The subagent's internal round number
    pub subagent_iteration: i32,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}

#[derive(Debug)]
pub(crate) struct CSubagentProgressPayloadOwned {
    node_id: CString,
    parent_tool_call_id: CString,
    subagent_tool_name: CString,
    subagent_tool_args: CString,
    subagent_status: CString,
    subagent_duration_ms: i64,
    subagent_iteration: i32,
    started_at: i64,
}

impl From<SubagentProgressPayload> for CSubagentProgressPayloadOwned {
    fn from(v: SubagentProgressPayload) -> Self {
        let SubagentProgressPayload {
            node_id,
            parent_tool_call_id,
            subagent_tool_name,
            subagent_tool_args,
            subagent_status,
            subagent_duration_ms,
            subagent_iteration,
            started_at,
        } = v;
        Self {
            node_id: node_id.into(),
            parent_tool_call_id: parent_tool_call_id.into(),
            subagent_tool_name: subagent_tool_name.into(),
            subagent_tool_args: subagent_tool_args.into(),
            subagent_status: subagent_status.into(),
            subagent_duration_ms,
            subagent_iteration,
            started_at,
        }
    }
}

impl ToFFI for CSubagentProgressPayloadOwned {
    type FFIType = CSubagentProgressPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSubagentProgressPayloadOwned {
            node_id,
            parent_tool_call_id,
            subagent_tool_name,
            subagent_tool_args,
            subagent_status,
            subagent_duration_ms,
            subagent_iteration,
            started_at,
        } = self;
        CSubagentProgressPayload {
            node_id: node_id.to_ffi_type(),
            parent_tool_call_id: parent_tool_call_id.to_ffi_type(),
            subagent_tool_name: subagent_tool_name.to_ffi_type(),
            subagent_tool_args: subagent_tool_args.to_ffi_type(),
            subagent_status: subagent_status.to_ffi_type(),
            subagent_duration_ms: *subagent_duration_ms,
            subagent_iteration: *subagent_iteration,
            started_at: *started_at,
        }
    }
}

/// `outputs` of a `SubagentFinished` conversation stream event
#[repr(C)]
pub struct CSubagentOutputs {
    /// The goal that was assigned to the subagent; empty when absent
    pub goal: *const c_char,
    /// The subagent's result; empty when absent
    pub result: *const c_char,
    /// Timeline of tool calls the subagent made, as a JSON array string;
    /// empty when absent
    pub subagent_tools_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CSubagentOutputsOwned {
    goal: CString,
    result: CString,
    subagent_tools_json: CString,
}

impl From<SubagentOutputs> for CSubagentOutputsOwned {
    fn from(v: SubagentOutputs) -> Self {
        let SubagentOutputs {
            goal,
            result,
            subagent_tools,
        } = v;
        Self {
            goal: goal.unwrap_or_default().into(),
            result: result.unwrap_or_default().into(),
            subagent_tools_json: subagent_tools
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CSubagentOutputsOwned {
    type FFIType = CSubagentOutputs;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSubagentOutputsOwned {
            goal,
            result,
            subagent_tools_json,
        } = self;
        CSubagentOutputs {
            goal: goal.to_ffi_type(),
            result: result.to_ffi_type(),
            subagent_tools_json: subagent_tools_json.to_ffi_type(),
        }
    }
}

/// Payload of a `SubagentFinished` conversation stream event
#[repr(C)]
pub struct CSubagentFinishedPayload {
    /// ID of the node that spawned the subagent
    pub node_id: *const c_char,
    /// Matches the `tool_use_id` of `SubagentStarted`
    pub tool_use_id: *const c_char,
    /// `succeeded` / `failed`
    pub status: *const c_char,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Total subagent duration in seconds
    pub elapsed_time: f64,
    /// Error description on failure
    pub error: *const c_char,
    /// Subagent result: `goal`, `result`, and the timeline of tool calls it
    /// made
    pub outputs: *const CSubagentOutputs,
}

#[derive(Debug)]
pub(crate) struct CSubagentFinishedPayloadOwned {
    node_id: CString,
    tool_use_id: CString,
    status: CString,
    started_at: i64,
    elapsed_time: f64,
    error: CString,
    outputs: CCow<CSubagentOutputsOwned>,
}

impl From<SubagentFinishedPayload> for CSubagentFinishedPayloadOwned {
    fn from(v: SubagentFinishedPayload) -> Self {
        let SubagentFinishedPayload {
            node_id,
            tool_use_id,
            status,
            started_at,
            elapsed_time,
            error,
            outputs,
        } = v;
        Self {
            node_id: node_id.into(),
            tool_use_id: tool_use_id.into(),
            status: status.into(),
            started_at,
            elapsed_time,
            error: error.into(),
            outputs: CCow::new(outputs),
        }
    }
}

impl ToFFI for CSubagentFinishedPayloadOwned {
    type FFIType = CSubagentFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CSubagentFinishedPayloadOwned {
            node_id,
            tool_use_id,
            status,
            started_at,
            elapsed_time,
            error,
            outputs,
        } = self;
        CSubagentFinishedPayload {
            node_id: node_id.to_ffi_type(),
            tool_use_id: tool_use_id.to_ffi_type(),
            status: status.to_ffi_type(),
            started_at: *started_at,
            elapsed_time: *elapsed_time,
            error: error.to_ffi_type(),
            outputs: outputs.to_ffi_type(),
        }
    }
}

/// Payload of an `AgentToolStarted` conversation stream event. When the
/// Agent delegates to another Agent as a tool, that inner run is reported
/// with the `AgentTool*` family — the shape mirrors the subagent events.
#[repr(C)]
pub struct CAgentToolStartedPayload {
    /// ID of the calling node
    pub node_id: *const c_char,
    /// Unique ID of this call; matches the finished event
    pub tool_use_id: *const c_char,
    /// Identifier of the Agent being called
    pub agent_tool_name: *const c_char,
    /// Display title; may be empty
    pub title: *const c_char,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Call arguments as a JSON string
    pub tool_args: *const c_char,
    /// Localized display name
    pub tool_name: *const c_char,
    /// Progress text; may be empty
    pub tips: *const c_char,
    /// Short tags; may be empty
    pub tip_chips: *const *const c_char,
    /// Number of tags in `tip_chips`
    pub num_tip_chips: usize,
    /// `true` if called during the thinking phase
    pub is_thinking: bool,
}

#[derive(Debug)]
pub(crate) struct CAgentToolStartedPayloadOwned {
    node_id: CString,
    tool_use_id: CString,
    agent_tool_name: CString,
    title: CString,
    started_at: i64,
    tool_args: CString,
    tool_name: CString,
    tips: CString,
    tip_chips: CVec<CString>,
    is_thinking: bool,
}

impl From<AgentToolStartedPayload> for CAgentToolStartedPayloadOwned {
    fn from(v: AgentToolStartedPayload) -> Self {
        let AgentToolStartedPayload {
            node_id,
            tool_use_id,
            agent_tool_name,
            title,
            started_at,
            tool_args,
            tool_name,
            tips,
            tip_chips,
            is_thinking,
        } = v;
        Self {
            node_id: node_id.into(),
            tool_use_id: tool_use_id.into(),
            agent_tool_name: agent_tool_name.into(),
            title: title.into(),
            started_at,
            tool_args: tool_args.into(),
            tool_name: tool_name.into(),
            tips: tips.into(),
            tip_chips: tip_chips.into(),
            is_thinking,
        }
    }
}

impl ToFFI for CAgentToolStartedPayloadOwned {
    type FFIType = CAgentToolStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAgentToolStartedPayloadOwned {
            node_id,
            tool_use_id,
            agent_tool_name,
            title,
            started_at,
            tool_args,
            tool_name,
            tips,
            tip_chips,
            is_thinking,
        } = self;
        CAgentToolStartedPayload {
            node_id: node_id.to_ffi_type(),
            tool_use_id: tool_use_id.to_ffi_type(),
            agent_tool_name: agent_tool_name.to_ffi_type(),
            title: title.to_ffi_type(),
            started_at: *started_at,
            tool_args: tool_args.to_ffi_type(),
            tool_name: tool_name.to_ffi_type(),
            tips: tips.to_ffi_type(),
            tip_chips: tip_chips.to_ffi_type(),
            num_tip_chips: tip_chips.len(),
            is_thinking: *is_thinking,
        }
    }
}

/// Payload of an `AgentToolProgress` conversation stream event, emitted for
/// each inner tool call the delegated Agent makes.
#[repr(C)]
pub struct CAgentToolProgressPayload {
    /// ID of the calling node
    pub node_id: *const c_char,
    /// `tool_use_id` of the owning `AgentToolStarted` event
    pub parent_tool_call_id: *const c_char,
    /// Identifier of the Agent being called
    pub agent_tool_name: *const c_char,
    /// Name of the inner tool the delegated Agent called
    pub inner_tool_name: *const c_char,
    /// Arguments of that inner call, as a JSON string
    pub inner_tool_args: *const c_char,
    /// Status of the inner call: `running` / `succeeded` / `failed`
    pub status: *const c_char,
    /// Duration of the inner call in milliseconds
    pub duration_ms: i64,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// `true` if during the thinking phase
    pub is_thinking: bool,
}

#[derive(Debug)]
pub(crate) struct CAgentToolProgressPayloadOwned {
    node_id: CString,
    parent_tool_call_id: CString,
    agent_tool_name: CString,
    inner_tool_name: CString,
    inner_tool_args: CString,
    status: CString,
    duration_ms: i64,
    started_at: i64,
    is_thinking: bool,
}

impl From<AgentToolProgressPayload> for CAgentToolProgressPayloadOwned {
    fn from(v: AgentToolProgressPayload) -> Self {
        let AgentToolProgressPayload {
            node_id,
            parent_tool_call_id,
            agent_tool_name,
            inner_tool_name,
            inner_tool_args,
            status,
            duration_ms,
            started_at,
            is_thinking,
        } = v;
        Self {
            node_id: node_id.into(),
            parent_tool_call_id: parent_tool_call_id.into(),
            agent_tool_name: agent_tool_name.into(),
            inner_tool_name: inner_tool_name.into(),
            inner_tool_args: inner_tool_args.into(),
            status: status.into(),
            duration_ms,
            started_at,
            is_thinking,
        }
    }
}

impl ToFFI for CAgentToolProgressPayloadOwned {
    type FFIType = CAgentToolProgressPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAgentToolProgressPayloadOwned {
            node_id,
            parent_tool_call_id,
            agent_tool_name,
            inner_tool_name,
            inner_tool_args,
            status,
            duration_ms,
            started_at,
            is_thinking,
        } = self;
        CAgentToolProgressPayload {
            node_id: node_id.to_ffi_type(),
            parent_tool_call_id: parent_tool_call_id.to_ffi_type(),
            agent_tool_name: agent_tool_name.to_ffi_type(),
            inner_tool_name: inner_tool_name.to_ffi_type(),
            inner_tool_args: inner_tool_args.to_ffi_type(),
            status: status.to_ffi_type(),
            duration_ms: *duration_ms,
            started_at: *started_at,
            is_thinking: *is_thinking,
        }
    }
}

/// Payload of an `AgentToolFinished` conversation stream event
#[repr(C)]
pub struct CAgentToolFinishedPayload {
    /// ID of the calling node
    pub node_id: *const c_char,
    /// Matches the `tool_use_id` of `AgentToolStarted`
    pub tool_use_id: *const c_char,
    /// Identifier of the Agent being called
    pub agent_tool_name: *const c_char,
    /// `succeeded` / `failed`
    pub status: *const c_char,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Total duration in seconds
    pub elapsed_time: f64,
    /// Error description on failure
    pub error: *const c_char,
    /// Call arguments as a JSON string
    pub tool_args: *const c_char,
    /// Result of the delegated Agent, as a JSON string; empty when absent
    pub outputs_json: *const c_char,
    /// Tool category
    pub tool_type: *const c_char,
    /// Progress text; may be empty
    pub tips: *const c_char,
    /// Short tags; may be empty
    pub tip_chips: *const *const c_char,
    /// Number of tags in `tip_chips`
    pub num_tip_chips: usize,
    /// `true` if during the thinking phase
    pub is_thinking: bool,
}

#[derive(Debug)]
pub(crate) struct CAgentToolFinishedPayloadOwned {
    node_id: CString,
    tool_use_id: CString,
    agent_tool_name: CString,
    status: CString,
    started_at: i64,
    elapsed_time: f64,
    error: CString,
    tool_args: CString,
    outputs_json: CString,
    tool_type: CString,
    tips: CString,
    tip_chips: CVec<CString>,
    is_thinking: bool,
}

impl From<AgentToolFinishedPayload> for CAgentToolFinishedPayloadOwned {
    fn from(v: AgentToolFinishedPayload) -> Self {
        let AgentToolFinishedPayload {
            node_id,
            tool_use_id,
            agent_tool_name,
            status,
            started_at,
            elapsed_time,
            error,
            tool_args,
            outputs,
            tool_type,
            tips,
            tip_chips,
            is_thinking,
        } = v;
        Self {
            node_id: node_id.into(),
            tool_use_id: tool_use_id.into(),
            agent_tool_name: agent_tool_name.into(),
            status: status.into(),
            started_at,
            elapsed_time,
            error: error.into(),
            tool_args: tool_args.into(),
            outputs_json: outputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
            tool_type: tool_type.into(),
            tips: tips.into(),
            tip_chips: tip_chips.into(),
            is_thinking,
        }
    }
}

impl ToFFI for CAgentToolFinishedPayloadOwned {
    type FFIType = CAgentToolFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CAgentToolFinishedPayloadOwned {
            node_id,
            tool_use_id,
            agent_tool_name,
            status,
            started_at,
            elapsed_time,
            error,
            tool_args,
            outputs_json,
            tool_type,
            tips,
            tip_chips,
            is_thinking,
        } = self;
        CAgentToolFinishedPayload {
            node_id: node_id.to_ffi_type(),
            tool_use_id: tool_use_id.to_ffi_type(),
            agent_tool_name: agent_tool_name.to_ffi_type(),
            status: status.to_ffi_type(),
            started_at: *started_at,
            elapsed_time: *elapsed_time,
            error: error.to_ffi_type(),
            tool_args: tool_args.to_ffi_type(),
            outputs_json: outputs_json.to_ffi_type(),
            tool_type: tool_type.to_ffi_type(),
            tips: tips.to_ffi_type(),
            tip_chips: tip_chips.to_ffi_type(),
            num_tip_chips: tip_chips.len(),
            is_thinking: *is_thinking,
        }
    }
}

/// Payload of a `QueryMasked` conversation stream event — sensitive content
/// in the user query was masked before processing. Display `masked_query`
/// instead of the original query.
#[repr(C)]
pub struct CQueryMaskedPayload {
    /// The original user query
    pub raw_query: *const c_char,
    /// The masked query
    pub masked_query: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CQueryMaskedPayloadOwned {
    raw_query: CString,
    masked_query: CString,
}

impl From<QueryMaskedPayload> for CQueryMaskedPayloadOwned {
    fn from(v: QueryMaskedPayload) -> Self {
        let QueryMaskedPayload {
            raw_query,
            masked_query,
        } = v;
        Self {
            raw_query: raw_query.into(),
            masked_query: masked_query.into(),
        }
    }
}

impl ToFFI for CQueryMaskedPayloadOwned {
    type FFIType = CQueryMaskedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CQueryMaskedPayloadOwned {
            raw_query,
            masked_query,
        } = self;
        CQueryMaskedPayload {
            raw_query: raw_query.to_ffi_type(),
            masked_query: masked_query.to_ffi_type(),
        }
    }
}

/// Payload of a `PlanChanged` conversation stream event — the Agent created
/// or updated its task plan.
#[repr(C)]
pub struct CPlanChangedPayload {
    /// ID of the planning node
    pub node_id: *const c_char,
    /// Time of the change, Unix timestamp in seconds
    pub started_at: i64,
    /// The current plan content, as a JSON string; empty when absent
    pub outputs_json: *const c_char,
    /// Identifies the planning tool
    pub tool_name: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CPlanChangedPayloadOwned {
    node_id: CString,
    started_at: i64,
    outputs_json: CString,
    tool_name: CString,
}

impl From<PlanChangedPayload> for CPlanChangedPayloadOwned {
    fn from(v: PlanChangedPayload) -> Self {
        let PlanChangedPayload {
            node_id,
            started_at,
            outputs,
            tool_name,
        } = v;
        Self {
            node_id: node_id.into(),
            started_at,
            outputs_json: outputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
            tool_name: tool_name.into(),
        }
    }
}

impl ToFFI for CPlanChangedPayloadOwned {
    type FFIType = CPlanChangedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CPlanChangedPayloadOwned {
            node_id,
            started_at,
            outputs_json,
            tool_name,
        } = self;
        CPlanChangedPayload {
            node_id: node_id.to_ffi_type(),
            started_at: *started_at,
            outputs_json: outputs_json.to_ffi_type(),
            tool_name: tool_name.to_ffi_type(),
        }
    }
}

/// Payload of a `ContextCompressStarted` conversation stream event, marking
/// the start of a context-compression pass triggered by a long
/// conversation. Unlike other events, `started_at` here is an RFC 3339
/// string.
#[repr(C)]
pub struct CContextCompressStartedPayload {
    /// Start time, RFC 3339
    pub started_at: *const c_char,
    /// Compression input summary, as a JSON string; empty when absent
    pub inputs_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CContextCompressStartedPayloadOwned {
    started_at: CString,
    inputs_json: CString,
}

impl From<ContextCompressStartedPayload> for CContextCompressStartedPayloadOwned {
    fn from(v: ContextCompressStartedPayload) -> Self {
        let ContextCompressStartedPayload { started_at, inputs } = v;
        Self {
            started_at: started_at.into(),
            inputs_json: inputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CContextCompressStartedPayloadOwned {
    type FFIType = CContextCompressStartedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CContextCompressStartedPayloadOwned {
            started_at,
            inputs_json,
        } = self;
        CContextCompressStartedPayload {
            started_at: started_at.to_ffi_type(),
            inputs_json: inputs_json.to_ffi_type(),
        }
    }
}

/// Payload of a `ContextCompressFinished` conversation stream event. Unlike
/// other events, `created_at` here is an RFC 3339 string.
#[repr(C)]
pub struct CContextCompressFinishedPayload {
    /// Finish time, RFC 3339
    pub created_at: *const c_char,
    /// Compression input summary, as a JSON string; empty when absent
    pub inputs_json: *const c_char,
    /// Compression result summary, as a JSON string; empty when absent
    pub outputs_json: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CContextCompressFinishedPayloadOwned {
    created_at: CString,
    inputs_json: CString,
    outputs_json: CString,
}

impl From<ContextCompressFinishedPayload> for CContextCompressFinishedPayloadOwned {
    fn from(v: ContextCompressFinishedPayload) -> Self {
        let ContextCompressFinishedPayload {
            created_at,
            inputs,
            outputs,
        } = v;
        Self {
            created_at: created_at.into(),
            inputs_json: inputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
            outputs_json: outputs
                .map(|v| serde_json::to_string(&v).unwrap_or_default())
                .unwrap_or_default()
                .into(),
        }
    }
}

impl ToFFI for CContextCompressFinishedPayloadOwned {
    type FFIType = CContextCompressFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CContextCompressFinishedPayloadOwned {
            created_at,
            inputs_json,
            outputs_json,
        } = self;
        CContextCompressFinishedPayload {
            created_at: created_at.to_ffi_type(),
            inputs_json: inputs_json.to_ffi_type(),
            outputs_json: outputs_json.to_ffi_type(),
        }
    }
}

/// Payload of a `ChatFinished` conversation stream event, observed once all
/// `Message` events for this round have been sent, shortly before
/// `WorkflowFinished`
#[repr(C)]
pub struct CChatFinishedPayload {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: *const c_char,
    /// Message ID of this round
    pub message_id: *const c_char,
    /// Empty string in every run observed so far
    pub error: *const c_char,
    /// Empty string in every run observed so far
    pub error_message: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CChatFinishedPayloadOwned {
    chat_id: i64,
    chat_uid: CString,
    message_id: CString,
    error: CString,
    error_message: CString,
}

impl From<ChatFinishedPayload> for CChatFinishedPayloadOwned {
    fn from(v: ChatFinishedPayload) -> Self {
        let ChatFinishedPayload {
            chat_id,
            chat_uid,
            message_id,
            error,
            error_message,
        } = v;
        Self {
            chat_id,
            chat_uid: chat_uid.into(),
            message_id: message_id.into(),
            error: error.into(),
            error_message: error_message.into(),
        }
    }
}

impl ToFFI for CChatFinishedPayloadOwned {
    type FFIType = CChatFinishedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CChatFinishedPayloadOwned {
            chat_id,
            chat_uid,
            message_id,
            error,
            error_message,
        } = self;
        CChatFinishedPayload {
            chat_id: *chat_id,
            chat_uid: chat_uid.to_ffi_type(),
            message_id: message_id.to_ffi_type(),
            error: error.to_ffi_type(),
            error_message: error_message.to_ffi_type(),
        }
    }
}

/// Payload of a `ChatTitleUpdated` conversation stream event — the server
/// auto-generates a short title for the conversation as a UI convenience.
/// Can arrive before *or* after `WorkflowFinished`; not tied to the run's
/// outcome.
#[repr(C)]
pub struct CChatTitleUpdatedPayload {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: *const c_char,
    /// Where the title came from, e.g. `"ai_generated"`
    pub source: *const c_char,
    /// The new (possibly truncated) title
    pub title: *const c_char,
    /// Unix timestamp in seconds
    pub updated_at: i64,
}

#[derive(Debug)]
pub(crate) struct CChatTitleUpdatedPayloadOwned {
    chat_id: i64,
    chat_uid: CString,
    source: CString,
    title: CString,
    updated_at: i64,
}

impl From<ChatTitleUpdatedPayload> for CChatTitleUpdatedPayloadOwned {
    fn from(v: ChatTitleUpdatedPayload) -> Self {
        let ChatTitleUpdatedPayload {
            chat_id,
            chat_uid,
            source,
            title,
            updated_at,
        } = v;
        Self {
            chat_id,
            chat_uid: chat_uid.into(),
            source: source.into(),
            title: title.into(),
            updated_at,
        }
    }
}

impl ToFFI for CChatTitleUpdatedPayloadOwned {
    type FFIType = CChatTitleUpdatedPayload;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CChatTitleUpdatedPayloadOwned {
            chat_id,
            chat_uid,
            source,
            title,
            updated_at,
        } = self;
        CChatTitleUpdatedPayload {
            chat_id: *chat_id,
            chat_uid: chat_uid.to_ffi_type(),
            source: source.to_ffi_type(),
            title: title.to_ffi_type(),
            updated_at: *updated_at,
        }
    }
}

/// One event observed while streaming `lb_agent_context_conversation_streamed`
/// or `lb_agent_context_continue_conversation_streamed`.
///
/// This is a tagged union: `kind` tells you which one field below is
/// non-null; all others are always null. When `kind` is `Ping` (a
/// heartbeat with no payload), every field below is null.
#[repr(C)]
pub struct CConversationStreamEvent {
    /// Discriminant, tells you which field below is populated
    pub kind: CConversationStreamEventType,
    /// Non-null when `kind` is `ChatStarted`
    pub chat_started: *const CChatStartedPayload,
    /// Non-null when `kind` is `WorkflowStarted`, observed right after
    /// `ChatStarted` on every run seen so far
    pub workflow_started: *const CWorkflowStartedPayload,
    /// Non-null when `kind` is `Message`
    pub message: *const CMessagePayload,
    /// Non-null when `kind` is `ThinkingStarted`, the Agent entering the
    /// reasoning phase
    pub thinking_started: *const CThinkingStartedPayload,
    /// Non-null when `kind` is `ThinkingFinished`, the reasoning phase
    /// ending
    pub thinking_finished: *const CThinkingFinishedPayload,
    /// Non-null when `kind` is `NodeToolUseStarted`, an ordinary tool call
    /// starting
    pub node_tool_use_started: *const CNodeToolUseStartedPayload,
    /// Non-null when `kind` is `NodeToolUseFinished`, an ordinary tool call
    /// ending
    pub node_tool_use_finished: *const CNodeToolUseFinishedPayload,
    /// Non-null when `kind` is `SubagentStarted`, the Agent spawning a
    /// subagent to work on a sub-task
    pub subagent_started: *const CSubagentStartedPayload,
    /// Non-null when `kind` is `SubagentProgress`, the subagent calling one
    /// of its own tools
    pub subagent_progress: *const CSubagentProgressPayload,
    /// Non-null when `kind` is `SubagentFinished`, the subagent finishing
    /// its sub-task
    pub subagent_finished: *const CSubagentFinishedPayload,
    /// Non-null when `kind` is `AgentToolStarted`, the Agent delegating to
    /// another Agent as a tool
    pub agent_tool_started: *const CAgentToolStartedPayload,
    /// Non-null when `kind` is `AgentToolProgress`, the delegated Agent
    /// calling one of its own tools
    pub agent_tool_progress: *const CAgentToolProgressPayload,
    /// Non-null when `kind` is `AgentToolFinished`, the delegated Agent's
    /// run finishing
    pub agent_tool_finished: *const CAgentToolFinishedPayload,
    /// Non-null when `kind` is `HumanInteractionRequired`, carrying the
    /// run's outcome for an interrupted run. Unlike `WorkflowFinished`, this
    /// is emitted instead of (never alongside) `WorkflowFinished` for the
    /// same run
    pub human_interaction_required: *const CConversationResponse,
    /// Non-null when `kind` is `QueryMasked`, sensitive content in the user
    /// query having been masked before processing
    pub query_masked: *const CQueryMaskedPayload,
    /// Non-null when `kind` is `PlanChanged`, the Agent creating or
    /// updating its task plan
    pub plan_changed: *const CPlanChangedPayload,
    /// Non-null when `kind` is `ContextCompressStarted`, a
    /// context-compression pass starting
    pub context_compress_started: *const CContextCompressStartedPayload,
    /// Non-null when `kind` is `ContextCompressFinished`, a
    /// context-compression pass finishing
    pub context_compress_finished: *const CContextCompressFinishedPayload,
    /// Non-null when `kind` is `ChatFinished`, observed once all `Message`
    /// events for this round have been sent
    pub chat_finished: *const CChatFinishedPayload,
    /// Non-null when `kind` is `WorkflowFinished`, carrying the run's
    /// outcome — not necessarily the last event of the stream, since the
    /// server may still emit a few more housekeeping events before actually
    /// closing the connection
    pub workflow_finished: *const CConversationResponse,
    /// Non-null when `kind` is `ChatTitleUpdated`, the server auto-generating
    /// a short title for the conversation
    pub chat_title_updated: *const CChatTitleUpdatedPayload,
    /// Non-null when `kind` is `Other`; the SSE envelope's `event` field (the
    /// event type name)
    pub other_event: *const c_char,
    /// Non-null when `kind` is `Other`; raw JSON of an event type not
    /// recognized by this SDK version
    pub other_json: *const c_char,
}

pub(crate) enum CConversationStreamEventOwned {
    ChatStarted(CCow<CChatStartedPayloadOwned>),
    WorkflowStarted(CCow<CWorkflowStartedPayloadOwned>),
    Message(CCow<CMessagePayloadOwned>),
    Ping,
    ThinkingStarted(CCow<CThinkingStartedPayloadOwned>),
    ThinkingFinished(CCow<CThinkingFinishedPayloadOwned>),
    NodeToolUseStarted(CCow<CNodeToolUseStartedPayloadOwned>),
    NodeToolUseFinished(CCow<CNodeToolUseFinishedPayloadOwned>),
    SubagentStarted(CCow<CSubagentStartedPayloadOwned>),
    SubagentProgress(CCow<CSubagentProgressPayloadOwned>),
    SubagentFinished(CCow<CSubagentFinishedPayloadOwned>),
    AgentToolStarted(CCow<CAgentToolStartedPayloadOwned>),
    AgentToolProgress(CCow<CAgentToolProgressPayloadOwned>),
    AgentToolFinished(CCow<CAgentToolFinishedPayloadOwned>),
    HumanInteractionRequired(CCow<CConversationResponseOwned>),
    QueryMasked(CCow<CQueryMaskedPayloadOwned>),
    PlanChanged(CCow<CPlanChangedPayloadOwned>),
    ContextCompressStarted(CCow<CContextCompressStartedPayloadOwned>),
    ContextCompressFinished(CCow<CContextCompressFinishedPayloadOwned>),
    ChatFinished(CCow<CChatFinishedPayloadOwned>),
    WorkflowFinished(CCow<CConversationResponseOwned>),
    ChatTitleUpdated(CCow<CChatTitleUpdatedPayloadOwned>),
    Other { event: CString, json: CString },
}

impl From<ConversationStreamEvent> for CConversationStreamEventOwned {
    fn from(v: ConversationStreamEvent) -> Self {
        match v {
            ConversationStreamEvent::ChatStarted(payload) => Self::ChatStarted(CCow::new(payload)),
            ConversationStreamEvent::WorkflowStarted(payload) => {
                Self::WorkflowStarted(CCow::new(payload))
            }
            ConversationStreamEvent::Message(payload) => Self::Message(CCow::new(payload)),
            ConversationStreamEvent::Ping => Self::Ping,
            ConversationStreamEvent::ThinkingStarted(payload) => {
                Self::ThinkingStarted(CCow::new(payload))
            }
            ConversationStreamEvent::ThinkingFinished(payload) => {
                Self::ThinkingFinished(CCow::new(payload))
            }
            ConversationStreamEvent::NodeToolUseStarted(payload) => {
                Self::NodeToolUseStarted(CCow::new(payload))
            }
            ConversationStreamEvent::NodeToolUseFinished(payload) => {
                Self::NodeToolUseFinished(CCow::new(payload))
            }
            ConversationStreamEvent::SubagentStarted(payload) => {
                Self::SubagentStarted(CCow::new(payload))
            }
            ConversationStreamEvent::SubagentProgress(payload) => {
                Self::SubagentProgress(CCow::new(payload))
            }
            ConversationStreamEvent::SubagentFinished(payload) => {
                Self::SubagentFinished(CCow::new(payload))
            }
            ConversationStreamEvent::AgentToolStarted(payload) => {
                Self::AgentToolStarted(CCow::new(payload))
            }
            ConversationStreamEvent::AgentToolProgress(payload) => {
                Self::AgentToolProgress(CCow::new(payload))
            }
            ConversationStreamEvent::AgentToolFinished(payload) => {
                Self::AgentToolFinished(CCow::new(payload))
            }
            // Like `WorkflowFinished`, this carries a synthesized
            // `ConversationResponse` — it's the terminal event for
            // interrupted runs, which never emit `WorkflowFinished` at all.
            ConversationStreamEvent::HumanInteractionRequired(resp) => {
                Self::HumanInteractionRequired(CCow::new(resp))
            }
            ConversationStreamEvent::QueryMasked(payload) => Self::QueryMasked(CCow::new(payload)),
            ConversationStreamEvent::PlanChanged(payload) => Self::PlanChanged(CCow::new(payload)),
            ConversationStreamEvent::ContextCompressStarted(payload) => {
                Self::ContextCompressStarted(CCow::new(payload))
            }
            ConversationStreamEvent::ContextCompressFinished(payload) => {
                Self::ContextCompressFinished(CCow::new(payload))
            }
            ConversationStreamEvent::ChatFinished(payload) => {
                Self::ChatFinished(CCow::new(payload))
            }
            ConversationStreamEvent::WorkflowFinished(resp) => {
                Self::WorkflowFinished(CCow::new(resp))
            }
            ConversationStreamEvent::ChatTitleUpdated(payload) => {
                Self::ChatTitleUpdated(CCow::new(payload))
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
        // Every field besides `kind` defaults to null; each arm below
        // overrides only the one field relevant to its `kind`.
        fn base(kind: CConversationStreamEventType) -> CConversationStreamEvent {
            CConversationStreamEvent {
                kind,
                chat_started: std::ptr::null(),
                workflow_started: std::ptr::null(),
                message: std::ptr::null(),
                thinking_started: std::ptr::null(),
                thinking_finished: std::ptr::null(),
                node_tool_use_started: std::ptr::null(),
                node_tool_use_finished: std::ptr::null(),
                subagent_started: std::ptr::null(),
                subagent_progress: std::ptr::null(),
                subagent_finished: std::ptr::null(),
                agent_tool_started: std::ptr::null(),
                agent_tool_progress: std::ptr::null(),
                agent_tool_finished: std::ptr::null(),
                human_interaction_required: std::ptr::null(),
                query_masked: std::ptr::null(),
                plan_changed: std::ptr::null(),
                context_compress_started: std::ptr::null(),
                context_compress_finished: std::ptr::null(),
                chat_finished: std::ptr::null(),
                workflow_finished: std::ptr::null(),
                chat_title_updated: std::ptr::null(),
                other_event: std::ptr::null(),
                other_json: std::ptr::null(),
            }
        }

        match self {
            Self::ChatStarted(payload) => CConversationStreamEvent {
                chat_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ChatStarted)
            },
            Self::WorkflowStarted(payload) => CConversationStreamEvent {
                workflow_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::WorkflowStarted)
            },
            Self::Message(payload) => CConversationStreamEvent {
                message: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::Message)
            },
            Self::Ping => base(CConversationStreamEventType::Ping),
            Self::ThinkingStarted(payload) => CConversationStreamEvent {
                thinking_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ThinkingStarted)
            },
            Self::ThinkingFinished(payload) => CConversationStreamEvent {
                thinking_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ThinkingFinished)
            },
            Self::NodeToolUseStarted(payload) => CConversationStreamEvent {
                node_tool_use_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::NodeToolUseStarted)
            },
            Self::NodeToolUseFinished(payload) => CConversationStreamEvent {
                node_tool_use_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::NodeToolUseFinished)
            },
            Self::SubagentStarted(payload) => CConversationStreamEvent {
                subagent_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::SubagentStarted)
            },
            Self::SubagentProgress(payload) => CConversationStreamEvent {
                subagent_progress: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::SubagentProgress)
            },
            Self::SubagentFinished(payload) => CConversationStreamEvent {
                subagent_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::SubagentFinished)
            },
            Self::AgentToolStarted(payload) => CConversationStreamEvent {
                agent_tool_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::AgentToolStarted)
            },
            Self::AgentToolProgress(payload) => CConversationStreamEvent {
                agent_tool_progress: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::AgentToolProgress)
            },
            Self::AgentToolFinished(payload) => CConversationStreamEvent {
                agent_tool_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::AgentToolFinished)
            },
            Self::HumanInteractionRequired(resp) => CConversationStreamEvent {
                human_interaction_required: resp.to_ffi_type(),
                ..base(CConversationStreamEventType::HumanInteractionRequired)
            },
            Self::QueryMasked(payload) => CConversationStreamEvent {
                query_masked: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::QueryMasked)
            },
            Self::PlanChanged(payload) => CConversationStreamEvent {
                plan_changed: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::PlanChanged)
            },
            Self::ContextCompressStarted(payload) => CConversationStreamEvent {
                context_compress_started: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ContextCompressStarted)
            },
            Self::ContextCompressFinished(payload) => CConversationStreamEvent {
                context_compress_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ContextCompressFinished)
            },
            Self::ChatFinished(payload) => CConversationStreamEvent {
                chat_finished: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ChatFinished)
            },
            Self::WorkflowFinished(resp) => CConversationStreamEvent {
                workflow_finished: resp.to_ffi_type(),
                ..base(CConversationStreamEventType::WorkflowFinished)
            },
            Self::ChatTitleUpdated(payload) => CConversationStreamEvent {
                chat_title_updated: payload.to_ffi_type(),
                ..base(CConversationStreamEventType::ChatTitleUpdated)
            },
            Self::Other { event, json } => CConversationStreamEvent {
                other_event: event.to_ffi_type(),
                other_json: json.to_ffi_type(),
                ..base(CConversationStreamEventType::Other)
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
