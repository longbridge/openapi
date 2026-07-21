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

/// Payload of a `message` stream event — an incremental text chunk. This is
/// the highest-frequency event; concatenate `text` fragments in arrival
/// order.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct MessagePayload {
    pub text: String,
    pub message_type: String,
    pub key: String,
    pub started_at: i64,
    pub stage: String,
    pub stage_title: String,
    pub stage_finished_title: String,
    pub outputs: Option<JsonValue>,
}

impl From<longbridge::agent::MessagePayload> for MessagePayload {
    fn from(v: longbridge::agent::MessagePayload) -> Self {
        Self {
            text: v.text,
            message_type: v.message_type,
            key: v.key,
            started_at: v.started_at,
            stage: v.stage,
            stage_title: v.stage_title,
            stage_finished_title: v.stage_finished_title,
            outputs: v.outputs.map(JsonValue),
        }
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

/// Payload of a `thinking_started` stream event — the Agent has entered the
/// reasoning phase (analyzing the question, planning tool calls). Between
/// this and `ConversationStreamEvent`'s `thinking_finished`, `message` events
/// with `message_type == "think"` and tool-call events may arrive.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ThinkingStartedPayload {
    pub started_at: i64,
}

impl From<longbridge::agent::ThinkingStartedPayload> for ThinkingStartedPayload {
    fn from(v: longbridge::agent::ThinkingStartedPayload) -> Self {
        Self {
            started_at: v.started_at,
        }
    }
}

/// Payload of a `thinking_finished` stream event — the reasoning phase is
/// over; answer text (`message` with `message_type == "answer"`) follows.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ThinkingFinishedPayload {
    pub finished_at: i64,
    pub elapsed_time: i32,
}

impl From<longbridge::agent::ThinkingFinishedPayload> for ThinkingFinishedPayload {
    fn from(v: longbridge::agent::ThinkingFinishedPayload) -> Self {
        Self {
            finished_at: v.finished_at,
            elapsed_time: v.elapsed_time,
        }
    }
}

/// Payload of a `node_tool_use_started` stream event — an ordinary tool call
/// has started. Match it to its `node_tool_use_finished` counterpart by
/// `tool_use_id`.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct NodeToolUseStartedPayload {
    pub tool_use_id: String,
    pub tool_name: String,
    pub tool_func_name: String,
    pub tool_args: String,
    pub tips: String,
    pub tip_chips: Vec<String>,
    pub iteration: i32,
    pub started_at: i64,
}

impl From<longbridge::agent::NodeToolUseStartedPayload> for NodeToolUseStartedPayload {
    fn from(v: longbridge::agent::NodeToolUseStartedPayload) -> Self {
        Self {
            tool_use_id: v.tool_use_id,
            tool_name: v.tool_name,
            tool_func_name: v.tool_func_name,
            tool_args: v.tool_args,
            tips: v.tips,
            tip_chips: v.tip_chips,
            iteration: v.iteration,
            started_at: v.started_at,
        }
    }
}

/// `outputs` of a `NodeToolUseFinishedPayload` — only carries fields meant
/// for display
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct NodeToolUseOutputs {
    pub references: Option<Vec<Reference>>,
    pub reference_domains: Option<Vec<String>>,
    pub query: Option<String>,
    pub text: Option<String>,
    pub tool_args: Option<JsonValue>,
    pub data: Option<JsonValue>,
}

impl From<longbridge::agent::NodeToolUseOutputs> for NodeToolUseOutputs {
    fn from(v: longbridge::agent::NodeToolUseOutputs) -> Self {
        Self {
            references: v
                .references
                .map(|refs| refs.into_iter().map(Into::into).collect()),
            reference_domains: v.reference_domains,
            query: v.query,
            text: v.text,
            tool_args: v.tool_args.map(JsonValue),
            data: v.data.map(JsonValue),
        }
    }
}

/// Payload of a `node_tool_use_finished` stream event — the tool call has
/// ended.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct NodeToolUseFinishedPayload {
    pub tool_use_id: String,
    pub status: String,
    pub error: String,
    pub elapsed_time: f64,
    pub started_at: i64,
    pub tool_name: String,
    pub tool_func_name: String,
    pub tool_args: String,
    pub tool_type: String,
    pub tips: String,
    pub tip_chips: Vec<String>,
    pub iteration: i32,
    pub is_thinking: bool,
    pub outputs: NodeToolUseOutputs,
}

impl From<longbridge::agent::NodeToolUseFinishedPayload> for NodeToolUseFinishedPayload {
    fn from(v: longbridge::agent::NodeToolUseFinishedPayload) -> Self {
        Self {
            tool_use_id: v.tool_use_id,
            status: v.status,
            error: v.error,
            elapsed_time: v.elapsed_time,
            started_at: v.started_at,
            tool_name: v.tool_name,
            tool_func_name: v.tool_func_name,
            tool_args: v.tool_args,
            tool_type: v.tool_type,
            tips: v.tips,
            tip_chips: v.tip_chips,
            iteration: v.iteration,
            is_thinking: v.is_thinking,
            outputs: v.outputs.into(),
        }
    }
}

/// Payload of a `subagent_started` stream event. When the Agent spawns a
/// subagent to work on a sub-task, the subagent's lifecycle is reported with
/// this dedicated event family instead of `node_tool_use_*`.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct SubagentStartedPayload {
    pub node_id: String,
    pub tool_use_id: String,
    pub started_at: i64,
    pub goal: String,
    pub prompt: String,
    pub subagent_id: String,
    pub tools: Vec<JsonValue>,
}

impl From<longbridge::agent::SubagentStartedPayload> for SubagentStartedPayload {
    fn from(v: longbridge::agent::SubagentStartedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            started_at: v.started_at,
            goal: v.goal,
            prompt: v.prompt,
            subagent_id: v.subagent_id,
            tools: v.tools.into_iter().map(JsonValue).collect(),
        }
    }
}

/// Payload of a `subagent_progress` stream event, emitted every time the
/// subagent calls one of its own tools. Use it to render a live timeline
/// inside the subagent card.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct SubagentProgressPayload {
    pub node_id: String,
    pub parent_tool_call_id: String,
    pub subagent_tool_name: String,
    pub subagent_tool_args: String,
    pub subagent_status: String,
    pub subagent_duration_ms: i64,
    pub subagent_iteration: i32,
    pub started_at: i64,
}

impl From<longbridge::agent::SubagentProgressPayload> for SubagentProgressPayload {
    fn from(v: longbridge::agent::SubagentProgressPayload) -> Self {
        Self {
            node_id: v.node_id,
            parent_tool_call_id: v.parent_tool_call_id,
            subagent_tool_name: v.subagent_tool_name,
            subagent_tool_args: v.subagent_tool_args,
            subagent_status: v.subagent_status,
            subagent_duration_ms: v.subagent_duration_ms,
            subagent_iteration: v.subagent_iteration,
            started_at: v.started_at,
        }
    }
}

/// `outputs` of a `SubagentFinishedPayload`
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct SubagentOutputs {
    pub goal: Option<String>,
    pub result: Option<String>,
    pub subagent_tools: Option<Vec<JsonValue>>,
}

impl From<longbridge::agent::SubagentOutputs> for SubagentOutputs {
    fn from(v: longbridge::agent::SubagentOutputs) -> Self {
        Self {
            goal: v.goal,
            result: v.result,
            subagent_tools: v
                .subagent_tools
                .map(|tools| tools.into_iter().map(JsonValue).collect()),
        }
    }
}

/// Payload of a `subagent_finished` stream event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct SubagentFinishedPayload {
    pub node_id: String,
    pub tool_use_id: String,
    pub status: String,
    pub started_at: i64,
    pub elapsed_time: f64,
    pub error: String,
    pub outputs: SubagentOutputs,
}

impl From<longbridge::agent::SubagentFinishedPayload> for SubagentFinishedPayload {
    fn from(v: longbridge::agent::SubagentFinishedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            status: v.status,
            started_at: v.started_at,
            elapsed_time: v.elapsed_time,
            error: v.error,
            outputs: v.outputs.into(),
        }
    }
}

/// Payload of an `agent_tool_started` stream event. When the Agent delegates
/// to another Agent as a tool, that inner run is reported with the
/// `agent_tool_*` family — the shape mirrors the subagent events.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct AgentToolStartedPayload {
    pub node_id: String,
    pub tool_use_id: String,
    pub agent_tool_name: String,
    pub title: String,
    pub started_at: i64,
    pub tool_args: String,
    pub tool_name: String,
    pub tips: String,
    pub tip_chips: Vec<String>,
    pub is_thinking: bool,
}

impl From<longbridge::agent::AgentToolStartedPayload> for AgentToolStartedPayload {
    fn from(v: longbridge::agent::AgentToolStartedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            agent_tool_name: v.agent_tool_name,
            title: v.title,
            started_at: v.started_at,
            tool_args: v.tool_args,
            tool_name: v.tool_name,
            tips: v.tips,
            tip_chips: v.tip_chips,
            is_thinking: v.is_thinking,
        }
    }
}

/// Payload of an `agent_tool_progress` stream event, emitted for each inner
/// tool call the delegated Agent makes.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct AgentToolProgressPayload {
    pub node_id: String,
    pub parent_tool_call_id: String,
    pub agent_tool_name: String,
    pub inner_tool_name: String,
    pub inner_tool_args: String,
    pub status: String,
    pub duration_ms: i64,
    pub started_at: i64,
    pub is_thinking: bool,
}

impl From<longbridge::agent::AgentToolProgressPayload> for AgentToolProgressPayload {
    fn from(v: longbridge::agent::AgentToolProgressPayload) -> Self {
        Self {
            node_id: v.node_id,
            parent_tool_call_id: v.parent_tool_call_id,
            agent_tool_name: v.agent_tool_name,
            inner_tool_name: v.inner_tool_name,
            inner_tool_args: v.inner_tool_args,
            status: v.status,
            duration_ms: v.duration_ms,
            started_at: v.started_at,
            is_thinking: v.is_thinking,
        }
    }
}

/// Payload of an `agent_tool_finished` stream event
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct AgentToolFinishedPayload {
    pub node_id: String,
    pub tool_use_id: String,
    pub agent_tool_name: String,
    pub status: String,
    pub started_at: i64,
    pub elapsed_time: f64,
    pub error: String,
    pub tool_args: String,
    pub outputs: Option<JsonValue>,
    pub tool_type: String,
    pub tips: String,
    pub tip_chips: Vec<String>,
    pub is_thinking: bool,
}

impl From<longbridge::agent::AgentToolFinishedPayload> for AgentToolFinishedPayload {
    fn from(v: longbridge::agent::AgentToolFinishedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            agent_tool_name: v.agent_tool_name,
            status: v.status,
            started_at: v.started_at,
            elapsed_time: v.elapsed_time,
            error: v.error,
            tool_args: v.tool_args,
            outputs: v.outputs.map(JsonValue),
            tool_type: v.tool_type,
            tips: v.tips,
            tip_chips: v.tip_chips,
            is_thinking: v.is_thinking,
        }
    }
}

/// Payload of a `query_masked` stream event — sensitive content in the user
/// query was masked before processing. Display `masked_query` instead of the
/// original query.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct QueryMaskedPayload {
    pub raw_query: String,
    pub masked_query: String,
}

impl From<longbridge::agent::QueryMaskedPayload> for QueryMaskedPayload {
    fn from(v: longbridge::agent::QueryMaskedPayload) -> Self {
        Self {
            raw_query: v.raw_query,
            masked_query: v.masked_query,
        }
    }
}

/// Payload of a `plan_changed` stream event — the Agent created or updated
/// its task plan.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct PlanChangedPayload {
    pub node_id: String,
    pub started_at: i64,
    pub outputs: Option<JsonValue>,
    pub tool_name: String,
}

impl From<longbridge::agent::PlanChangedPayload> for PlanChangedPayload {
    fn from(v: longbridge::agent::PlanChangedPayload) -> Self {
        Self {
            node_id: v.node_id,
            started_at: v.started_at,
            outputs: v.outputs.map(JsonValue),
            tool_name: v.tool_name,
        }
    }
}

/// Payload of a `context_compress_started` stream event, marking the start of
/// a context-compression pass triggered by a long conversation. Unlike other
/// events, the timestamp here is an RFC 3339 string.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ContextCompressStartedPayload {
    pub started_at: String,
    pub inputs: Option<JsonValue>,
}

impl From<longbridge::agent::ContextCompressStartedPayload> for ContextCompressStartedPayload {
    fn from(v: longbridge::agent::ContextCompressStartedPayload) -> Self {
        Self {
            started_at: v.started_at,
            inputs: v.inputs.map(JsonValue),
        }
    }
}

/// Payload of a `context_compress_finished` stream event. Unlike other
/// events, the timestamp here is an RFC 3339 string.
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ContextCompressFinishedPayload {
    pub created_at: String,
    pub inputs: Option<JsonValue>,
    pub outputs: Option<JsonValue>,
}

impl From<longbridge::agent::ContextCompressFinishedPayload> for ContextCompressFinishedPayload {
    fn from(v: longbridge::agent::ContextCompressFinishedPayload) -> Self {
        Self {
            created_at: v.created_at,
            inputs: v.inputs.map(JsonValue),
            outputs: v.outputs.map(JsonValue),
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
/// `"message"`, `"ping"`, `"thinking_started"`, `"thinking_finished"`,
/// `"node_tool_use_started"`, `"node_tool_use_finished"`, `"subagent_started"`,
/// `"subagent_progress"`, `"subagent_finished"`, `"agent_tool_started"`,
/// `"agent_tool_progress"`, `"agent_tool_finished"`,
/// `"human_interaction_required"`, `"query_masked"`, `"plan_changed"`,
/// `"context_compress_started"`, `"context_compress_finished"`,
/// `"chat_finished"`, `"workflow_finished"`, `"chat_title_updated"`,
/// `"other"`) and exactly one of the fields below sharing that name is set,
/// matching `kind` — except `"ping"`, a heartbeat with no payload, for which
/// every payload field is `None`. When `kind` is `"other"`, `other_event`
/// additionally carries the SSE envelope's `event` field (the event type
/// name).
#[pyclass(get_all, skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub(crate) struct ConversationStreamEvent {
    pub kind: String,
    pub chat_started: Option<ChatStartedPayload>,
    pub workflow_started: Option<WorkflowStartedPayload>,
    pub message: Option<MessagePayload>,
    pub thinking_started: Option<ThinkingStartedPayload>,
    pub thinking_finished: Option<ThinkingFinishedPayload>,
    pub node_tool_use_started: Option<NodeToolUseStartedPayload>,
    pub node_tool_use_finished: Option<NodeToolUseFinishedPayload>,
    pub subagent_started: Option<SubagentStartedPayload>,
    pub subagent_progress: Option<SubagentProgressPayload>,
    pub subagent_finished: Option<SubagentFinishedPayload>,
    pub agent_tool_started: Option<AgentToolStartedPayload>,
    pub agent_tool_progress: Option<AgentToolProgressPayload>,
    pub agent_tool_finished: Option<AgentToolFinishedPayload>,
    pub human_interaction_required: Option<ConversationResponse>,
    pub query_masked: Option<QueryMaskedPayload>,
    pub plan_changed: Option<PlanChangedPayload>,
    pub context_compress_started: Option<ContextCompressStartedPayload>,
    pub context_compress_finished: Option<ContextCompressFinishedPayload>,
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
                ..Default::default()
            },
            E::WorkflowStarted(payload) => Self {
                kind: "workflow_started".to_string(),
                workflow_started: Some(payload.into()),
                ..Default::default()
            },
            E::Message(payload) => Self {
                kind: "message".to_string(),
                message: Some(payload.into()),
                ..Default::default()
            },
            E::Ping => Self {
                kind: "ping".to_string(),
                ..Default::default()
            },
            E::ThinkingStarted(payload) => Self {
                kind: "thinking_started".to_string(),
                thinking_started: Some(payload.into()),
                ..Default::default()
            },
            E::ThinkingFinished(payload) => Self {
                kind: "thinking_finished".to_string(),
                thinking_finished: Some(payload.into()),
                ..Default::default()
            },
            E::NodeToolUseStarted(payload) => Self {
                kind: "node_tool_use_started".to_string(),
                node_tool_use_started: Some(payload.into()),
                ..Default::default()
            },
            E::NodeToolUseFinished(payload) => Self {
                kind: "node_tool_use_finished".to_string(),
                node_tool_use_finished: Some(payload.into()),
                ..Default::default()
            },
            E::SubagentStarted(payload) => Self {
                kind: "subagent_started".to_string(),
                subagent_started: Some(payload.into()),
                ..Default::default()
            },
            E::SubagentProgress(payload) => Self {
                kind: "subagent_progress".to_string(),
                subagent_progress: Some(payload.into()),
                ..Default::default()
            },
            E::SubagentFinished(payload) => Self {
                kind: "subagent_finished".to_string(),
                subagent_finished: Some(payload.into()),
                ..Default::default()
            },
            E::AgentToolStarted(payload) => Self {
                kind: "agent_tool_started".to_string(),
                agent_tool_started: Some(payload.into()),
                ..Default::default()
            },
            E::AgentToolProgress(payload) => Self {
                kind: "agent_tool_progress".to_string(),
                agent_tool_progress: Some(payload.into()),
                ..Default::default()
            },
            E::AgentToolFinished(payload) => Self {
                kind: "agent_tool_finished".to_string(),
                agent_tool_finished: Some(payload.into()),
                ..Default::default()
            },
            E::HumanInteractionRequired(resp) => Self {
                kind: "human_interaction_required".to_string(),
                human_interaction_required: Some(resp.into()),
                ..Default::default()
            },
            E::QueryMasked(payload) => Self {
                kind: "query_masked".to_string(),
                query_masked: Some(payload.into()),
                ..Default::default()
            },
            E::PlanChanged(payload) => Self {
                kind: "plan_changed".to_string(),
                plan_changed: Some(payload.into()),
                ..Default::default()
            },
            E::ContextCompressStarted(payload) => Self {
                kind: "context_compress_started".to_string(),
                context_compress_started: Some(payload.into()),
                ..Default::default()
            },
            E::ContextCompressFinished(payload) => Self {
                kind: "context_compress_finished".to_string(),
                context_compress_finished: Some(payload.into()),
                ..Default::default()
            },
            E::ChatFinished(payload) => Self {
                kind: "chat_finished".to_string(),
                chat_finished: Some(payload.into()),
                ..Default::default()
            },
            E::WorkflowFinished(resp) => Self {
                kind: "workflow_finished".to_string(),
                workflow_finished: Some(resp.into()),
                ..Default::default()
            },
            E::ChatTitleUpdated(payload) => Self {
                kind: "chat_title_updated".to_string(),
                chat_title_updated: Some(payload.into()),
                ..Default::default()
            },
            E::Other { event, data } => Self {
                kind: "other".to_string(),
                other_event: Some(event),
                other: Some(JsonValue(data)),
                ..Default::default()
            },
        }
    }
}
