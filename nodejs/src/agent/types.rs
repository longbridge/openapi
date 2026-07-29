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

/// Payload of a `message` stream event — an incremental text chunk. This is
/// the highest-frequency event; concatenate `text` fragments in arrival
/// order.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct MessagePayload {
    /// Incremental text fragment
    pub text: String,
    /// `answer` — final answer text; `think` — reasoning process; `process`
    /// — stage progress description
    pub message_type: String,
    /// Identifier of the stream segment this fragment belongs to. Fragments
    /// with the same `key` form one continuous block — group by `key` when
    /// rendering
    pub key: String,
    /// Time this segment started, Unix timestamp in seconds
    pub started_at: i64,
    /// Stage identifier; only present when `messageType` is `"process"`
    pub stage: String,
    /// Stage title while running; only present when `messageType` is
    /// `"process"`
    pub stage_title: String,
    /// Stage title after it finishes; only present when `messageType` is
    /// `"process"`
    pub stage_finished_title: String,
    /// Extra payload attached to the fragment; usually absent
    pub outputs: Option<serde_json::Value>,
}
impl From<lb::MessagePayload> for MessagePayload {
    fn from(v: lb::MessagePayload) -> Self {
        Self {
            text: v.text,
            message_type: v.message_type,
            key: v.key,
            started_at: v.started_at,
            stage: v.stage,
            stage_title: v.stage_title,
            stage_finished_title: v.stage_finished_title,
            outputs: v.outputs,
        }
    }
}

/// `inputs` of a `workflow_started` stream event
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct WorkflowStartedInputs {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: String,
    /// Message ID of this round
    pub message_id: String,
    /// The question that was asked
    pub query: String,
}
impl From<lb::WorkflowStartedInputs> for WorkflowStartedInputs {
    fn from(v: lb::WorkflowStartedInputs) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct WorkflowStartedPayload {
    /// Whether this run's answer was served from a cache
    pub hit_cache: bool,
    /// Echoes the run's inputs
    pub inputs: WorkflowStartedInputs,
    /// Unix timestamp in seconds
    pub started_at: i64,
    /// Internal workflow run ID
    pub workflow_id: i64,
}
impl From<lb::WorkflowStartedPayload> for WorkflowStartedPayload {
    fn from(v: lb::WorkflowStartedPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ChatFinishedPayload {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: String,
    /// Message ID of this round
    pub message_id: String,
    /// Empty string in every run observed so far
    pub error: String,
    /// Empty string in every run observed so far
    pub error_message: String,
}
impl From<lb::ChatFinishedPayload> for ChatFinishedPayload {
    fn from(v: lb::ChatFinishedPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ChatTitleUpdatedPayload {
    /// ID of the owning conversation
    pub chat_id: i64,
    /// Conversation identifier
    pub chat_uid: String,
    /// Where the title came from, e.g. `"ai_generated"`
    pub source: String,
    /// The new (possibly truncated) title
    pub title: String,
    /// Unix timestamp in seconds
    pub updated_at: i64,
}
impl From<lb::ChatTitleUpdatedPayload> for ChatTitleUpdatedPayload {
    fn from(v: lb::ChatTitleUpdatedPayload) -> Self {
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
/// this and `ThinkingFinished`, `Message` events with `messageType ==
/// "think"` and tool-call events may arrive.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ThinkingStartedPayload {
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}
impl From<lb::ThinkingStartedPayload> for ThinkingStartedPayload {
    fn from(v: lb::ThinkingStartedPayload) -> Self {
        Self {
            started_at: v.started_at,
        }
    }
}

/// Payload of a `thinking_finished` stream event — the reasoning phase is
/// over; answer text (`Message` with `messageType == "answer"`) follows.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ThinkingFinishedPayload {
    /// Finish time, Unix timestamp in seconds
    pub finished_at: i64,
    /// Reasoning duration in seconds
    pub elapsed_time: i32,
}
impl From<lb::ThinkingFinishedPayload> for ThinkingFinishedPayload {
    fn from(v: lb::ThinkingFinishedPayload) -> Self {
        Self {
            finished_at: v.finished_at,
            elapsed_time: v.elapsed_time,
        }
    }
}

/// Payload of a `node_tool_use_started` stream event — an ordinary tool call
/// has started. Match it to its `NodeToolUseFinished` counterpart by
/// `toolUseId`.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct NodeToolUseStartedPayload {
    /// Unique ID of this call; matches the finished event
    pub tool_use_id: String,
    /// Localized display name of the tool
    pub tool_name: String,
    /// Locale-stable tool identifier; use this for logic keyed on the tool
    /// kind
    pub tool_func_name: String,
    /// Call arguments as a JSON string
    pub tool_args: String,
    /// Progress text suitable for direct display, e.g. `"Searching the
    /// web…"`
    pub tips: String,
    /// Short tags accompanying `tips`; may be omitted
    pub tip_chips: Vec<String>,
    /// Round number. Calls in the same round (same `iteration`) run in
    /// parallel
    pub iteration: i32,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}
impl From<lb::NodeToolUseStartedPayload> for NodeToolUseStartedPayload {
    fn from(v: lb::NodeToolUseStartedPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct NodeToolUseOutputs {
    /// Sources referenced by the tool result
    pub references: Option<Vec<Reference>>,
    /// Domains of the referenced sources
    pub reference_domains: Option<Vec<String>>,
    /// The query the tool executed
    pub query: Option<String>,
    /// Raw response text of the tool
    pub text: Option<String>,
    /// Parsed request arguments
    pub tool_args: Option<serde_json::Value>,
    /// Structured result; present only for selected tools
    pub data: Option<serde_json::Value>,
}
impl From<lb::NodeToolUseOutputs> for NodeToolUseOutputs {
    fn from(v: lb::NodeToolUseOutputs) -> Self {
        Self {
            references: v
                .references
                .map(|refs| refs.into_iter().map(Into::into).collect()),
            reference_domains: v.reference_domains,
            query: v.query,
            text: v.text,
            tool_args: v.tool_args,
            data: v.data,
        }
    }
}

/// Payload of a `node_tool_use_finished` stream event — the tool call has
/// ended.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct NodeToolUseFinishedPayload {
    /// Matches the `toolUseId` of the started event
    pub tool_use_id: String,
    /// `succeeded` / `failed`
    pub status: String,
    /// Error description on failure
    pub error: String,
    /// Call duration in seconds
    pub elapsed_time: f64,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Localized display name
    pub tool_name: String,
    /// Locale-stable tool identifier
    pub tool_func_name: String,
    /// Call arguments as a JSON string
    pub tool_args: String,
    /// Tool category
    pub tool_type: String,
    /// Progress text
    pub tips: String,
    /// Short tags; may be omitted
    pub tip_chips: Vec<String>,
    /// Round number
    pub iteration: i32,
    /// `true` if the call happened during the thinking phase
    pub is_thinking: bool,
    /// Filtered call results, for display
    pub outputs: NodeToolUseOutputs,
}
impl From<lb::NodeToolUseFinishedPayload> for NodeToolUseFinishedPayload {
    fn from(v: lb::NodeToolUseFinishedPayload) -> Self {
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
/// this dedicated event family instead of `nodeToolUse*`.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SubagentStartedPayload {
    /// ID of the node that spawned the subagent
    pub node_id: String,
    /// Unique ID of this spawn; matches the finished event
    pub tool_use_id: String,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Goal assigned to the subagent
    pub goal: String,
    /// Full task prompt given to the subagent
    pub prompt: String,
    /// Subagent identifier; may be omitted
    pub subagent_id: String,
    /// Tools granted to the subagent; may be omitted
    pub tools: Vec<serde_json::Value>,
}
impl From<lb::SubagentStartedPayload> for SubagentStartedPayload {
    fn from(v: lb::SubagentStartedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            started_at: v.started_at,
            goal: v.goal,
            prompt: v.prompt,
            subagent_id: v.subagent_id,
            tools: v.tools,
        }
    }
}

/// Payload of a `subagent_progress` stream event, emitted every time the
/// subagent calls one of its own tools. Use it to render a live timeline
/// inside the subagent card.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SubagentProgressPayload {
    /// ID of the node that spawned the subagent
    pub node_id: String,
    /// `toolUseId` of the owning `SubagentStarted` event
    pub parent_tool_call_id: String,
    /// Name of the tool the subagent called
    pub subagent_tool_name: String,
    /// Arguments of that call, as a JSON string
    pub subagent_tool_args: String,
    /// Status of that call: `running` / `succeeded` / `failed`
    pub subagent_status: String,
    /// Duration of that call in milliseconds
    pub subagent_duration_ms: i64,
    /// The subagent's internal round number
    pub subagent_iteration: i32,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
}
impl From<lb::SubagentProgressPayload> for SubagentProgressPayload {
    fn from(v: lb::SubagentProgressPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SubagentOutputs {
    /// The goal that was assigned to the subagent
    pub goal: Option<String>,
    /// The subagent's result
    pub result: Option<String>,
    /// Timeline of tool calls the subagent made
    pub subagent_tools: Option<Vec<serde_json::Value>>,
}
impl From<lb::SubagentOutputs> for SubagentOutputs {
    fn from(v: lb::SubagentOutputs) -> Self {
        Self {
            goal: v.goal,
            result: v.result,
            subagent_tools: v.subagent_tools,
        }
    }
}

/// Payload of a `subagent_finished` stream event
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SubagentFinishedPayload {
    /// ID of the node that spawned the subagent
    pub node_id: String,
    /// Matches the `toolUseId` of `SubagentStarted`
    pub tool_use_id: String,
    /// `succeeded` / `failed`
    pub status: String,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Total subagent duration in seconds
    pub elapsed_time: f64,
    /// Error description on failure
    pub error: String,
    /// Subagent result: `goal`, `result`, and the timeline of tool calls it
    /// made
    pub outputs: SubagentOutputs,
}
impl From<lb::SubagentFinishedPayload> for SubagentFinishedPayload {
    fn from(v: lb::SubagentFinishedPayload) -> Self {
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
/// `agentTool*` family — the shape mirrors the subagent events.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AgentToolStartedPayload {
    /// ID of the calling node
    pub node_id: String,
    /// Unique ID of this call; matches the finished event
    pub tool_use_id: String,
    /// Identifier of the Agent being called
    pub agent_tool_name: String,
    /// Display title; may be omitted
    pub title: String,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Call arguments as a JSON string
    pub tool_args: String,
    /// Localized display name
    pub tool_name: String,
    /// Progress text; may be omitted
    pub tips: String,
    /// Short tags; may be omitted
    pub tip_chips: Vec<String>,
    /// `true` if called during the thinking phase
    pub is_thinking: bool,
}
impl From<lb::AgentToolStartedPayload> for AgentToolStartedPayload {
    fn from(v: lb::AgentToolStartedPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AgentToolProgressPayload {
    /// ID of the calling node
    pub node_id: String,
    /// `toolUseId` of the owning `AgentToolStarted` event
    pub parent_tool_call_id: String,
    /// Identifier of the Agent being called
    pub agent_tool_name: String,
    /// Name of the inner tool the delegated Agent called
    pub inner_tool_name: String,
    /// Arguments of that inner call, as a JSON string
    pub inner_tool_args: String,
    /// Status of the inner call: `running` / `succeeded` / `failed`
    pub status: String,
    /// Duration of the inner call in milliseconds
    pub duration_ms: i64,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// `true` if during the thinking phase
    pub is_thinking: bool,
}
impl From<lb::AgentToolProgressPayload> for AgentToolProgressPayload {
    fn from(v: lb::AgentToolProgressPayload) -> Self {
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
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct AgentToolFinishedPayload {
    /// ID of the calling node
    pub node_id: String,
    /// Matches the `toolUseId` of `AgentToolStarted`
    pub tool_use_id: String,
    /// Identifier of the Agent being called
    pub agent_tool_name: String,
    /// `succeeded` / `failed`
    pub status: String,
    /// Start time, Unix timestamp in seconds
    pub started_at: i64,
    /// Total duration in seconds
    pub elapsed_time: f64,
    /// Error description on failure
    pub error: String,
    /// Call arguments as a JSON string
    pub tool_args: String,
    /// Result of the delegated Agent
    pub outputs: Option<serde_json::Value>,
    /// Tool category
    pub tool_type: String,
    /// Progress text; may be omitted
    pub tips: String,
    /// Short tags; may be omitted
    pub tip_chips: Vec<String>,
    /// `true` if during the thinking phase
    pub is_thinking: bool,
}
impl From<lb::AgentToolFinishedPayload> for AgentToolFinishedPayload {
    fn from(v: lb::AgentToolFinishedPayload) -> Self {
        Self {
            node_id: v.node_id,
            tool_use_id: v.tool_use_id,
            agent_tool_name: v.agent_tool_name,
            status: v.status,
            started_at: v.started_at,
            elapsed_time: v.elapsed_time,
            error: v.error,
            tool_args: v.tool_args,
            outputs: v.outputs,
            tool_type: v.tool_type,
            tips: v.tips,
            tip_chips: v.tip_chips,
            is_thinking: v.is_thinking,
        }
    }
}

/// Payload of a `query_masked` stream event — sensitive content in the user
/// query was masked before processing. Display `maskedQuery` instead of the
/// original query.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct QueryMaskedPayload {
    /// The original user query
    pub raw_query: String,
    /// The masked query
    pub masked_query: String,
}
impl From<lb::QueryMaskedPayload> for QueryMaskedPayload {
    fn from(v: lb::QueryMaskedPayload) -> Self {
        Self {
            raw_query: v.raw_query,
            masked_query: v.masked_query,
        }
    }
}

/// Payload of a `plan_changed` stream event — the Agent created or updated
/// its task plan.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct PlanChangedPayload {
    /// ID of the planning node
    pub node_id: String,
    /// Time of the change, Unix timestamp in seconds
    pub started_at: i64,
    /// The current plan content
    pub outputs: Option<serde_json::Value>,
    /// Identifies the planning tool
    pub tool_name: String,
}
impl From<lb::PlanChangedPayload> for PlanChangedPayload {
    fn from(v: lb::PlanChangedPayload) -> Self {
        Self {
            node_id: v.node_id,
            started_at: v.started_at,
            outputs: v.outputs,
            tool_name: v.tool_name,
        }
    }
}

/// Payload of a `context_compress_started` stream event, marking the start
/// of a context-compression pass triggered by a long conversation. Unlike
/// other events, the timestamp here is an RFC 3339 string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ContextCompressStartedPayload {
    /// Start time, RFC 3339
    pub started_at: String,
    /// Compression input summary
    pub inputs: Option<serde_json::Value>,
}
impl From<lb::ContextCompressStartedPayload> for ContextCompressStartedPayload {
    fn from(v: lb::ContextCompressStartedPayload) -> Self {
        Self {
            started_at: v.started_at,
            inputs: v.inputs,
        }
    }
}

/// Payload of a `context_compress_finished` stream event. Unlike other
/// events, the timestamp here is an RFC 3339 string.
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ContextCompressFinishedPayload {
    /// Finish time, RFC 3339
    pub created_at: String,
    /// Compression input summary
    pub inputs: Option<serde_json::Value>,
    /// Compression result summary
    pub outputs: Option<serde_json::Value>,
}
impl From<lb::ContextCompressFinishedPayload> for ContextCompressFinishedPayload {
    fn from(v: lb::ContextCompressFinishedPayload) -> Self {
        Self {
            created_at: v.created_at,
            inputs: v.inputs,
            outputs: v.outputs,
        }
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
/// `"chat_started" | "workflow_started" | "message" | "ping" |
/// "thinking_started" | "thinking_finished" | "node_tool_use_started" |
/// "node_tool_use_finished" | "subagent_started" | "subagent_progress" |
/// "subagent_finished" | "agent_tool_started" | "agent_tool_progress" |
/// "agent_tool_finished" | "human_interaction_required" | "query_masked" |
/// "plan_changed" | "context_compress_started" | "context_compress_finished" |
/// "chat_finished" | "workflow_finished" | "chat_title_updated" | "other"`,
/// and exactly one of `chatStarted` / `workflowStarted` / `message` /
/// `thinkingStarted` / `thinkingFinished` / `nodeToolUseStarted` /
/// `nodeToolUseFinished` / `subagentStarted` / `subagentProgress` /
/// `subagentFinished` / `agentToolStarted` / `agentToolProgress` /
/// `agentToolFinished` / `humanInteractionRequired` / `queryMasked` /
/// `planChanged` / `contextCompressStarted` / `contextCompressFinished` /
/// `chatFinished` / `workflowFinished` / `chatTitleUpdated` / `other` is set,
/// matching `kind` — except `"ping"`, a heartbeat with no payload, for which
/// every payload field is `None`. When `kind` is `"other"`, `otherEvent`
/// additionally carries the SSE envelope's `event` field (the event type
/// name).
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct ConversationStreamEvent {
    /// Discriminant: one of `"chat_started"`, `"workflow_started"`,
    /// `"message"`, `"ping"`, `"thinking_started"`, `"thinking_finished"`,
    /// `"node_tool_use_started"`, `"node_tool_use_finished"`,
    /// `"subagent_started"`, `"subagent_progress"`, `"subagent_finished"`,
    /// `"agent_tool_started"`, `"agent_tool_progress"`,
    /// `"agent_tool_finished"`, `"human_interaction_required"`,
    /// `"query_masked"`, `"plan_changed"`, `"context_compress_started"`,
    /// `"context_compress_finished"`, `"chat_finished"`,
    /// `"workflow_finished"`, `"chat_title_updated"`, or `"other"`
    pub kind: String,
    /// Set when `kind` is `"chat_started"`
    pub chat_started: Option<ChatStartedPayload>,
    /// Set when `kind` is `"workflow_started"`
    pub workflow_started: Option<WorkflowStartedPayload>,
    /// Set when `kind` is `"message"`
    pub message: Option<MessagePayload>,
    /// Set when `kind` is `"thinking_started"`
    pub thinking_started: Option<ThinkingStartedPayload>,
    /// Set when `kind` is `"thinking_finished"`
    pub thinking_finished: Option<ThinkingFinishedPayload>,
    /// Set when `kind` is `"node_tool_use_started"`
    pub node_tool_use_started: Option<NodeToolUseStartedPayload>,
    /// Set when `kind` is `"node_tool_use_finished"`
    pub node_tool_use_finished: Option<NodeToolUseFinishedPayload>,
    /// Set when `kind` is `"subagent_started"`
    pub subagent_started: Option<SubagentStartedPayload>,
    /// Set when `kind` is `"subagent_progress"`
    pub subagent_progress: Option<SubagentProgressPayload>,
    /// Set when `kind` is `"subagent_finished"`
    pub subagent_finished: Option<SubagentFinishedPayload>,
    /// Set when `kind` is `"agent_tool_started"`
    pub agent_tool_started: Option<AgentToolStartedPayload>,
    /// Set when `kind` is `"agent_tool_progress"`
    pub agent_tool_progress: Option<AgentToolProgressPayload>,
    /// Set when `kind` is `"agent_tool_finished"`
    pub agent_tool_finished: Option<AgentToolFinishedPayload>,
    /// Set when `kind` is `"human_interaction_required"`, carrying the run's
    /// outcome for an interrupted run — the same `ConversationResponse` shape
    /// `workflowFinished` carries for the other outcomes. Unlike
    /// `workflowFinished`, this is set instead of (never alongside)
    /// `workflowFinished` for the same run.
    pub human_interaction_required: Option<ConversationResponse>,
    /// Set when `kind` is `"query_masked"`
    pub query_masked: Option<QueryMaskedPayload>,
    /// Set when `kind` is `"plan_changed"`
    pub plan_changed: Option<PlanChangedPayload>,
    /// Set when `kind` is `"context_compress_started"`
    pub context_compress_started: Option<ContextCompressStartedPayload>,
    /// Set when `kind` is `"context_compress_finished"`
    pub context_compress_finished: Option<ContextCompressFinishedPayload>,
    /// Set when `kind` is `"chat_finished"`
    pub chat_finished: Option<ChatFinishedPayload>,
    /// Set when `kind` is `"workflow_finished"`, carrying the run's outcome
    /// — not necessarily the last event of the stream, since the server may
    /// still emit a few more housekeeping events (`kind` `"other"`) before
    /// actually closing the connection
    pub workflow_finished: Option<ConversationResponse>,
    /// Set when `kind` is `"chat_title_updated"`
    pub chat_title_updated: Option<ChatTitleUpdatedPayload>,
    /// Set when `kind` is `"other"` — the SSE envelope's `event` field (the
    /// event type name)
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
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::WorkflowStarted(payload) => Self {
                kind: "workflow_started".to_string(),
                chat_started: None,
                workflow_started: Some(payload.into()),
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::Message(payload) => Self {
                kind: "message".to_string(),
                chat_started: None,
                workflow_started: None,
                message: Some(payload.into()),
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::Ping => Self {
                kind: "ping".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ThinkingStarted(payload) => Self {
                kind: "thinking_started".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: Some(payload.into()),
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ThinkingFinished(payload) => Self {
                kind: "thinking_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: Some(payload.into()),
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::NodeToolUseStarted(payload) => Self {
                kind: "node_tool_use_started".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: Some(payload.into()),
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::NodeToolUseFinished(payload) => Self {
                kind: "node_tool_use_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: Some(payload.into()),
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::SubagentStarted(payload) => Self {
                kind: "subagent_started".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: Some(payload.into()),
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::SubagentProgress(payload) => Self {
                kind: "subagent_progress".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: Some(payload.into()),
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::SubagentFinished(payload) => Self {
                kind: "subagent_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: Some(payload.into()),
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::AgentToolStarted(payload) => Self {
                kind: "agent_tool_started".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: Some(payload.into()),
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::AgentToolProgress(payload) => Self {
                kind: "agent_tool_progress".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: Some(payload.into()),
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::AgentToolFinished(payload) => Self {
                kind: "agent_tool_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: Some(payload.into()),
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::HumanInteractionRequired(resp) => Self {
                kind: "human_interaction_required".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: Some(resp.into()),
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::QueryMasked(payload) => Self {
                kind: "query_masked".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: Some(payload.into()),
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::PlanChanged(payload) => Self {
                kind: "plan_changed".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: Some(payload.into()),
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ContextCompressStarted(payload) => Self {
                kind: "context_compress_started".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: Some(payload.into()),
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ContextCompressFinished(payload) => Self {
                kind: "context_compress_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: Some(payload.into()),
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ChatFinished(payload) => Self {
                kind: "chat_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: Some(payload.into()),
                workflow_finished: None,
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::WorkflowFinished(resp) => Self {
                kind: "workflow_finished".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: Some(resp.into()),
                chat_title_updated: None,
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::ChatTitleUpdated(payload) => Self {
                kind: "chat_title_updated".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: Some(payload.into()),
                other_event: None,
                other: None,
            },
            lb::ConversationStreamEvent::Other { event, data } => Self {
                kind: "other".to_string(),
                chat_started: None,
                workflow_started: None,
                message: None,
                thinking_started: None,
                thinking_finished: None,
                node_tool_use_started: None,
                node_tool_use_finished: None,
                subagent_started: None,
                subagent_progress: None,
                subagent_finished: None,
                agent_tool_started: None,
                agent_tool_progress: None,
                agent_tool_finished: None,
                human_interaction_required: None,
                query_masked: None,
                plan_changed: None,
                context_compress_started: None,
                context_compress_finished: None,
                chat_finished: None,
                workflow_finished: None,
                chat_title_updated: None,
                other_event: Some(event),
                other: Some(data),
            },
        }
    }
}
