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
        let error = (payload.status == ConversationStatus::Failed).then_some(AgentError {
            code: payload.error_code,
            message: payload.error_message,
        });
        Self {
            chat_uid,
            message_id,
            status: payload.status,
            answer: payload.outputs.answer.unwrap_or_default(),
            references: payload.outputs.references,
            elapsed_time: payload.elapsed_time,
            interrupt: None,
            error,
        }
    }

    /// Build a [`ConversationResponse`] from a streamed conversation's parts —
    /// `chat_uid`/`message_id` captured from an earlier `chat_started` event,
    /// and a `human_interaction_required` event's [`Interrupt`] payload.
    ///
    /// Unlike the succeeded/failed/stopped cases, an interrupted run doesn't
    /// emit `workflow_finished` at all — `human_interaction_required` is the
    /// terminal event of the stream instead, so this plays the same role
    /// [`Self::from_stream_parts`] plays for the other outcomes.
    pub(crate) fn from_stream_interrupt(
        started: Option<(String, String)>,
        interrupt: Interrupt,
    ) -> Self {
        let (chat_uid, message_id) = started.unwrap_or_default();
        Self {
            chat_uid,
            message_id,
            status: ConversationStatus::Interrupted,
            answer: String::new(),
            references: None,
            elapsed_time: 0.0,
            interrupt: Some(interrupt),
            error: None,
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

/// Payload of a `message` SSE event — an incremental text chunk. This is the
/// highest-frequency event; concatenate `text` fragments in arrival order.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MessagePayload {
    /// Incremental text fragment
    #[serde(default)]
    pub text: String,
    /// `answer` — final answer text; `think` — reasoning process; `process`
    /// — stage progress description
    #[serde(default, rename = "type")]
    pub message_type: String,
    /// Identifier of the stream segment this fragment belongs to. Fragments
    /// with the same `key` form one continuous block — group by `key` when
    /// rendering
    #[serde(default)]
    pub key: String,
    /// Time this segment started, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Stage identifier; only present when `message_type` is `"process"`
    #[serde(default)]
    pub stage: String,
    /// Stage title while running; only present when `message_type` is
    /// `"process"`
    #[serde(default)]
    pub stage_title: String,
    /// Stage title after it finishes; only present when `message_type` is
    /// `"process"`
    #[serde(default)]
    pub stage_finished_title: String,
    /// Extra payload attached to the fragment; usually absent
    #[serde(default)]
    pub outputs: Option<serde_json::Value>,
}

/// `outputs` of a `workflow_finished` SSE event
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkflowOutputs {
    /// Final answer text; present when the run succeeded
    #[serde(default)]
    pub answer: Option<String>,
    /// Sources referenced by the answer
    #[serde(default)]
    pub references: Option<Vec<Reference>>,
}

/// Payload of a `workflow_finished` SSE event. `status` is never
/// `interrupted` here — an interrupted run doesn't emit `workflow_finished`
/// at all; see [`ConversationStreamEvent::HumanInteractionRequired`].
#[derive(Debug, Clone, Deserialize)]
pub struct WorkflowFinishedPayload {
    /// Final run status: `succeeded` / `failed` / `stopped`
    pub status: ConversationStatus,
    /// Run duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Run outputs
    #[serde(default)]
    pub outputs: WorkflowOutputs,
    /// Localized error description; only present when `status` is `failed`
    #[serde(default)]
    pub error: String,
    /// Error code; only present when `status` is `failed`
    #[serde(default)]
    pub error_code: i32,
    /// User-facing error message; only present on failure
    #[serde(default)]
    pub error_message: String,
    /// Extra error context (e.g. `workflow_run_id`); may be omitted
    #[serde(default)]
    pub error_args: Option<serde_json::Value>,
    /// Process stages the run went through; for display only
    #[serde(default)]
    pub process_data: Vec<serde_json::Value>,
}

/// `inputs` of a `workflow_started` SSE event
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkflowStartedInputs {
    /// ID of the owning conversation
    #[serde(default)]
    pub chat_id: i64,
    /// Conversation identifier
    #[serde(default)]
    pub chat_uid: String,
    /// Message ID of this round (observed as a raw JSON number; accepts a
    /// string too, see [`ChatStartedPayload::message_id`])
    #[serde(
        default,
        deserialize_with = "crate::serde_utils::deserialize_string_or_int_as_string"
    )]
    pub message_id: String,
    /// The question that was asked
    #[serde(default)]
    pub query: String,
}

/// Payload of a `workflow_started` SSE event, observed right after
/// `chat_started`
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WorkflowStartedPayload {
    /// Whether this run's answer was served from a cache
    #[serde(default)]
    pub hit_cache: bool,
    /// Echoes the run's inputs
    #[serde(default)]
    pub inputs: WorkflowStartedInputs,
    /// Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Internal workflow run ID
    #[serde(default)]
    pub workflow_id: i64,
}

/// Payload of a `chat_finished` SSE event, observed once all `message` events
/// for this round have been sent, shortly before `workflow_finished`
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatFinishedPayload {
    /// ID of the owning conversation
    #[serde(default)]
    pub chat_id: i64,
    /// Conversation identifier
    #[serde(default)]
    pub chat_uid: String,
    /// Message ID of this round (observed as a raw JSON number; accepts a
    /// string too, see [`ChatStartedPayload::message_id`])
    #[serde(
        default,
        deserialize_with = "crate::serde_utils::deserialize_string_or_int_as_string"
    )]
    pub message_id: String,
    /// Error detail; empty on success
    #[serde(default)]
    pub error: String,
    /// User-facing error message; empty on success
    #[serde(default)]
    pub error_message: String,
}

/// Payload of a `chat_title_updated` SSE event — the server auto-generates a
/// short title for the conversation as a UI convenience. Can arrive before
/// *or* after `workflow_finished`; not tied to the run's outcome.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatTitleUpdatedPayload {
    /// ID of the owning conversation
    #[serde(default)]
    pub chat_id: i64,
    /// Conversation identifier
    #[serde(default)]
    pub chat_uid: String,
    /// Where the title came from, e.g. `"ai_generated"`
    #[serde(default)]
    pub source: String,
    /// The new (possibly truncated) title
    #[serde(default)]
    pub title: String,
    /// Unix timestamp in seconds
    #[serde(default)]
    pub updated_at: i64,
}

/// Payload of a `thinking_started` SSE event — the Agent has entered the
/// reasoning phase (analyzing the question, planning tool calls). Between
/// this and [`ConversationStreamEvent::ThinkingFinished`], `Message` events
/// with `message_type == "think"` and tool-call events may arrive.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ThinkingStartedPayload {
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
}

/// Payload of a `thinking_finished` SSE event — the reasoning phase is over;
/// answer text (`Message` with `message_type == "answer"`) follows.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ThinkingFinishedPayload {
    /// Finish time, Unix timestamp in seconds
    #[serde(default)]
    pub finished_at: i64,
    /// Reasoning duration in seconds
    #[serde(default)]
    pub elapsed_time: i32,
}

/// Payload of a `node_tool_use_started` SSE event — an ordinary tool call has
/// started. Match it to its `NodeToolUseFinished` counterpart by
/// `tool_use_id`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NodeToolUseStartedPayload {
    /// Unique ID of this call; matches the finished event
    #[serde(default)]
    pub tool_use_id: String,
    /// Localized display name of the tool
    #[serde(default)]
    pub tool_name: String,
    /// Locale-stable tool identifier; use this for logic keyed on the tool
    /// kind
    #[serde(default)]
    pub tool_func_name: String,
    /// Call arguments as a JSON string
    #[serde(default)]
    pub tool_args: String,
    /// Progress text suitable for direct display, e.g. `"Searching the
    /// web…"`
    #[serde(default)]
    pub tips: String,
    /// Short tags accompanying `tips`; may be omitted
    #[serde(default)]
    pub tip_chips: Vec<String>,
    /// Round number. Calls in the same round (same `iteration`) run in
    /// parallel
    #[serde(default)]
    pub iteration: i32,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
}

/// `outputs` of a [`NodeToolUseFinishedPayload`] — only carries fields meant
/// for display
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NodeToolUseOutputs {
    /// Sources referenced by the tool result
    #[serde(default)]
    pub references: Option<Vec<Reference>>,
    /// Domains of the referenced sources
    #[serde(default)]
    pub reference_domains: Option<Vec<String>>,
    /// The query the tool executed
    #[serde(default)]
    pub query: Option<String>,
    /// Raw response text of the tool
    #[serde(default)]
    pub text: Option<String>,
    /// Parsed request arguments
    #[serde(default)]
    pub tool_args: Option<serde_json::Value>,
    /// Structured result; present only for selected tools
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Payload of a `node_tool_use_finished` SSE event — the tool call has
/// ended.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NodeToolUseFinishedPayload {
    /// Matches the `tool_use_id` of the started event
    #[serde(default)]
    pub tool_use_id: String,
    /// `succeeded` / `failed`
    #[serde(default)]
    pub status: String,
    /// Error description on failure
    #[serde(default)]
    pub error: String,
    /// Call duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Localized display name
    #[serde(default)]
    pub tool_name: String,
    /// Locale-stable tool identifier
    #[serde(default)]
    pub tool_func_name: String,
    /// Call arguments as a JSON string
    #[serde(default)]
    pub tool_args: String,
    /// Tool category
    #[serde(default)]
    pub tool_type: String,
    /// Progress text
    #[serde(default)]
    pub tips: String,
    /// Short tags; may be omitted
    #[serde(default)]
    pub tip_chips: Vec<String>,
    /// Round number
    #[serde(default)]
    pub iteration: i32,
    /// `true` if the call happened during the thinking phase
    #[serde(default)]
    pub is_thinking: bool,
    /// Filtered call results, for display
    #[serde(default)]
    pub outputs: NodeToolUseOutputs,
}

/// Payload of a `subagent_started` SSE event. When the Agent spawns a
/// subagent to work on a sub-task, the subagent's lifecycle is reported with
/// this dedicated event family instead of `node_tool_use_*`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SubagentStartedPayload {
    /// ID of the node that spawned the subagent
    #[serde(default)]
    pub node_id: String,
    /// Unique ID of this spawn; matches the finished event
    #[serde(default)]
    pub tool_use_id: String,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Goal assigned to the subagent
    #[serde(default)]
    pub goal: String,
    /// Full task prompt given to the subagent
    #[serde(default)]
    pub prompt: String,
    /// Subagent identifier; may be omitted
    #[serde(default)]
    pub subagent_id: String,
    /// Tools granted to the subagent; may be omitted
    #[serde(default)]
    pub tools: Vec<serde_json::Value>,
}

/// Payload of a `subagent_progress` SSE event, emitted every time the
/// subagent calls one of its own tools. Use it to render a live timeline
/// inside the subagent card.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SubagentProgressPayload {
    /// ID of the node that spawned the subagent
    #[serde(default)]
    pub node_id: String,
    /// `tool_use_id` of the owning `SubagentStarted` event
    #[serde(default)]
    pub parent_tool_call_id: String,
    /// Name of the tool the subagent called
    #[serde(default)]
    pub subagent_tool_name: String,
    /// Arguments of that call, as a JSON string
    #[serde(default)]
    pub subagent_tool_args: String,
    /// Status of that call: `running` / `succeeded` / `failed`
    #[serde(default)]
    pub subagent_status: String,
    /// Duration of that call in milliseconds
    #[serde(default)]
    pub subagent_duration_ms: i64,
    /// The subagent's internal round number
    #[serde(default)]
    pub subagent_iteration: i32,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
}

/// `outputs` of a [`SubagentFinishedPayload`]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SubagentOutputs {
    /// The goal that was assigned to the subagent
    #[serde(default)]
    pub goal: Option<String>,
    /// The subagent's result
    #[serde(default)]
    pub result: Option<String>,
    /// Timeline of tool calls the subagent made
    #[serde(default)]
    pub subagent_tools: Option<Vec<serde_json::Value>>,
}

/// Payload of a `subagent_finished` SSE event
#[derive(Debug, Clone, Default, Deserialize)]
pub struct SubagentFinishedPayload {
    /// ID of the node that spawned the subagent
    #[serde(default)]
    pub node_id: String,
    /// Matches the `tool_use_id` of `SubagentStarted`
    #[serde(default)]
    pub tool_use_id: String,
    /// `succeeded` / `failed`
    #[serde(default)]
    pub status: String,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Total subagent duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Error description on failure
    #[serde(default)]
    pub error: String,
    /// Subagent result: `goal`, `result`, and the timeline of tool calls it
    /// made
    #[serde(default)]
    pub outputs: SubagentOutputs,
}

/// Payload of an `agent_tool_started` SSE event. When the Agent delegates to
/// another Agent as a tool, that inner run is reported with the
/// `agent_tool_*` family — the shape mirrors the subagent events.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AgentToolStartedPayload {
    /// ID of the calling node
    #[serde(default)]
    pub node_id: String,
    /// Unique ID of this call; matches the finished event
    #[serde(default)]
    pub tool_use_id: String,
    /// Identifier of the Agent being called
    #[serde(default)]
    pub agent_tool_name: String,
    /// Display title; may be omitted
    #[serde(default)]
    pub title: String,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Call arguments as a JSON string
    #[serde(default)]
    pub tool_args: String,
    /// Localized display name
    #[serde(default)]
    pub tool_name: String,
    /// Progress text; may be omitted
    #[serde(default)]
    pub tips: String,
    /// Short tags; may be omitted
    #[serde(default)]
    pub tip_chips: Vec<String>,
    /// `true` if called during the thinking phase
    #[serde(default)]
    pub is_thinking: bool,
}

/// Payload of an `agent_tool_progress` SSE event, emitted for each inner
/// tool call the delegated Agent makes.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AgentToolProgressPayload {
    /// ID of the calling node
    #[serde(default)]
    pub node_id: String,
    /// `tool_use_id` of the owning `AgentToolStarted` event
    #[serde(default)]
    pub parent_tool_call_id: String,
    /// Identifier of the Agent being called
    #[serde(default)]
    pub agent_tool_name: String,
    /// Name of the inner tool the delegated Agent called
    #[serde(default)]
    pub inner_tool_name: String,
    /// Arguments of that inner call, as a JSON string
    #[serde(default)]
    pub inner_tool_args: String,
    /// Status of the inner call: `running` / `succeeded` / `failed`
    #[serde(default)]
    pub status: String,
    /// Duration of the inner call in milliseconds
    #[serde(default)]
    pub duration_ms: i64,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// `true` if during the thinking phase
    #[serde(default)]
    pub is_thinking: bool,
}

/// Payload of an `agent_tool_finished` SSE event
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AgentToolFinishedPayload {
    /// ID of the calling node
    #[serde(default)]
    pub node_id: String,
    /// Matches the `tool_use_id` of `AgentToolStarted`
    #[serde(default)]
    pub tool_use_id: String,
    /// Identifier of the Agent being called
    #[serde(default)]
    pub agent_tool_name: String,
    /// `succeeded` / `failed`
    #[serde(default)]
    pub status: String,
    /// Start time, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// Total duration in seconds
    #[serde(default)]
    pub elapsed_time: f64,
    /// Error description on failure
    #[serde(default)]
    pub error: String,
    /// Call arguments as a JSON string
    #[serde(default)]
    pub tool_args: String,
    /// Result of the delegated Agent
    #[serde(default)]
    pub outputs: Option<serde_json::Value>,
    /// Tool category
    #[serde(default)]
    pub tool_type: String,
    /// Progress text; may be omitted
    #[serde(default)]
    pub tips: String,
    /// Short tags; may be omitted
    #[serde(default)]
    pub tip_chips: Vec<String>,
    /// `true` if during the thinking phase
    #[serde(default)]
    pub is_thinking: bool,
}

/// Payload of a `query_masked` SSE event — sensitive content in the user
/// query was masked before processing. Display `masked_query` instead of the
/// original query.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct QueryMaskedPayload {
    /// The original user query
    #[serde(default)]
    pub raw_query: String,
    /// The masked query
    #[serde(default)]
    pub masked_query: String,
}

/// Payload of a `plan_changed` SSE event — the Agent created or updated its
/// task plan.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlanChangedPayload {
    /// ID of the planning node
    #[serde(default)]
    pub node_id: String,
    /// Time of the change, Unix timestamp in seconds
    #[serde(default)]
    pub started_at: i64,
    /// The current plan content
    #[serde(default)]
    pub outputs: Option<serde_json::Value>,
    /// Identifies the planning tool. Carried as a top-level sibling of
    /// `data` in the raw SSE envelope rather than inside `data` itself.
    #[serde(default)]
    pub tool_name: String,
}

/// Payload of a `context_compress_started` SSE event, marking the start of a
/// context-compression pass triggered by a long conversation. Unlike other
/// events, the timestamp here is an RFC 3339 string.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContextCompressStartedPayload {
    /// Start time, RFC 3339
    #[serde(default)]
    pub started_at: String,
    /// Compression input summary
    #[serde(default)]
    pub inputs: Option<serde_json::Value>,
}

/// Payload of a `context_compress_finished` SSE event. Unlike other events,
/// the timestamp here is an RFC 3339 string.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ContextCompressFinishedPayload {
    /// Finish time, RFC 3339
    #[serde(default)]
    pub created_at: String,
    /// Compression input summary
    #[serde(default)]
    pub inputs: Option<serde_json::Value>,
    /// Compression result summary
    #[serde(default)]
    pub outputs: Option<serde_json::Value>,
}

/// One event observed while streaming
/// [`crate::AgentContext::conversation_streamed`]
/// or [`crate::AgentContext::continue_conversation_streamed`].
///
/// A run always begins with `ChatStarted` and ends with `ChatFinished`. What
/// happens in between depends on the outcome:
///
/// - Succeeded: `ChatStarted` → `WorkflowStarted` → `ThinkingStarted` →
///   `Message` (`message_type == "think"`) … → `NodeToolUseStarted` /
///   `NodeToolUseFinished` … → `ThinkingFinished` → `Message` (`message_type ==
///   "answer"`) … → `WorkflowFinished` (`status == "succeeded"`) →
///   `ChatFinished`
/// - Interrupted (the Agent needs your input; resume via
///   [`crate::AgentContext::continue_conversation_streamed`]): `ChatStarted` →
///   `WorkflowStarted` → … → `HumanInteractionRequired` → `ChatFinished`. An
///   interrupted run does **not** emit `WorkflowFinished`, and resuming it does
///   **not** emit `WorkflowStarted` again.
/// - Failed: `ChatStarted` → `WorkflowStarted` → … → `WorkflowFinished`
///   (`status == "failed"`) → `ChatFinished`
///
/// For a plain question-and-answer integration you only need to handle four
/// variants — everything else is optional progress display: `Message` with
/// `message_type == "answer"` (append `text` to the answer being displayed),
/// `HumanInteractionRequired` (show the questions and call
/// `continue_conversation`/`continue_conversation_streamed` with the
/// answers), `WorkflowFinished` (read the final outcome), and `ChatFinished`
/// (the stream is over).
#[derive(Debug, Clone)]
pub enum ConversationStreamEvent {
    /// The run has started
    ChatStarted(ChatStartedPayload),
    /// Observed right after `ChatStarted` on every run seen so far, see
    /// [`WorkflowStartedPayload`]'s docs. Not emitted when resuming an
    /// interrupted run.
    WorkflowStarted(WorkflowStartedPayload),
    /// An incremental piece of the answer
    Message(MessagePayload),
    /// A heartbeat with no payload, observed at arbitrary points in the
    /// stream (including in between `Message` chunks)
    Ping,
    /// The Agent has entered the reasoning phase
    ThinkingStarted(ThinkingStartedPayload),
    /// The reasoning phase is over
    ThinkingFinished(ThinkingFinishedPayload),
    /// An ordinary tool call has started
    NodeToolUseStarted(NodeToolUseStartedPayload),
    /// An ordinary tool call has ended
    NodeToolUseFinished(NodeToolUseFinishedPayload),
    /// The Agent has spawned a subagent to work on a sub-task
    SubagentStarted(SubagentStartedPayload),
    /// The subagent has called one of its own tools
    SubagentProgress(SubagentProgressPayload),
    /// The subagent has finished its sub-task
    SubagentFinished(SubagentFinishedPayload),
    /// The Agent has delegated to another Agent as a tool
    AgentToolStarted(AgentToolStartedPayload),
    /// The delegated Agent has called one of its own tools
    AgentToolProgress(AgentToolProgressPayload),
    /// The delegated Agent's run has finished
    AgentToolFinished(AgentToolFinishedPayload),
    /// The run is paused: the Agent needs more information or confirmation
    /// from you, carrying the interrupt to resume from via
    /// [`crate::AgentContext::continue_conversation_streamed`]. Unlike
    /// `WorkflowFinished`, this is emitted instead of (never alongside)
    /// `WorkflowFinished` for the same run.
    HumanInteractionRequired(ConversationResponse),
    /// Sensitive content in the user query was masked before processing
    QueryMasked(QueryMaskedPayload),
    /// The Agent created or updated its task plan
    PlanChanged(PlanChangedPayload),
    /// A context-compression pass has started (long conversations trigger
    /// this)
    ContextCompressStarted(ContextCompressStartedPayload),
    /// The context-compression pass has finished
    ContextCompressFinished(ContextCompressFinishedPayload),
    /// Observed once all `Message` events for this round have been sent, see
    /// [`ChatFinishedPayload`]'s docs
    ChatFinished(ChatFinishedPayload),
    /// The run finished successfully, with a failure, or stopped by the
    /// user, carrying the run's outcome. Never emitted for an interrupted
    /// run — see [`ConversationStreamEvent::HumanInteractionRequired`] for
    /// that case. Not necessarily the last event of the stream — the server
    /// may still emit a few more housekeeping events (e.g.
    /// [`ConversationStreamEvent::ChatTitleUpdated`]) before actually
    /// closing the connection.
    WorkflowFinished(ConversationResponse),
    /// The server auto-generating a short title for the conversation, see
    /// [`ChatTitleUpdatedPayload`]'s docs. Can arrive before *or* after
    /// [`ConversationStreamEvent::WorkflowFinished`].
    ChatTitleUpdated(ChatTitleUpdatedPayload),
    /// An event type not recognized by this SDK version, carried as raw JSON
    /// so callers aren't broken by future additions to the API. `event` is
    /// the SSE envelope's discriminator string, so callers can at least tell
    /// these apart instead of getting an opaque blob.
    Other {
        /// The SSE envelope's `event` field (the event type name)
        event: String,
        /// The SSE envelope's `data` field
        data: serde_json::Value,
    },
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
    fn deserialize_message_payload_with_full_fields() {
        // https://github.com/longbridge/developers/pull/1176
        let json =
            r#"{"text":"Tesla","type":"answer","key":"n_llm_1:answer","started_at":1752048000}"#;
        let payload: MessagePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.text, "Tesla");
        assert_eq!(payload.message_type, "answer");
        assert_eq!(payload.key, "n_llm_1:answer");
        assert_eq!(payload.started_at, 1752048000);
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
        assert!(resp.interrupt.is_none());
        assert!(resp.error.is_none());
    }

    #[test]
    fn deserialize_workflow_finished_payload_with_failure() {
        // Error info is top-level on the event, not nested under `outputs`
        // (unlike the blocking response's `ConversationResponse.error`).
        let json = r#"{"status":"failed","elapsed_time":0.8,"error":"upstream timeout","error_code":500,"error_message":"Something went wrong, please try again"}"#;
        let payload: WorkflowFinishedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.status, ConversationStatus::Failed);
        assert_eq!(payload.error, "upstream timeout");
        assert_eq!(payload.error_code, 500);
        assert_eq!(
            payload.error_message,
            "Something went wrong, please try again"
        );

        let resp = ConversationResponse::from_stream_parts(None, payload);
        assert_eq!(resp.status, ConversationStatus::Failed);
        let error = resp.error.expect("error");
        assert_eq!(error.code, 500);
        assert_eq!(error.message, "Something went wrong, please try again");
    }

    #[test]
    fn conversation_response_from_stream_interrupt() {
        // An interrupted run never emits `workflow_finished` — the
        // `human_interaction_required` event is the terminal one instead,
        // and carries an `Interrupt` shaped identically to the blocking
        // response's `interrupt` field.
        let json = r#"{"node_id":"n_ask_human","tool_call_id":"call_abc123","questions":[{"question":"Which time range would you like to check?","options":[{"description":"Past week"},{"description":"Past month"}],"multi_select":false}],"message_id":43,"chat_id":1001}"#;
        let interrupt: Interrupt = serde_json::from_str(json).unwrap();

        let resp = ConversationResponse::from_stream_interrupt(
            Some(("ct_9f2c1a5b".to_string(), "43".to_string())),
            interrupt,
        );
        assert_eq!(resp.chat_uid, "ct_9f2c1a5b");
        assert_eq!(resp.message_id, "43");
        assert_eq!(resp.status, ConversationStatus::Interrupted);
        let interrupt = resp.interrupt.expect("interrupt");
        assert_eq!(interrupt.node_id, "n_ask_human");
        assert_eq!(interrupt.questions.len(), 1);
    }

    #[test]
    fn deserialize_node_tool_use_finished_payload() {
        let json = r#"{"tool_use_id":"call_abc123","status":"succeeded","elapsed_time":1.42,"tool_name":"Web Search","tool_func_name":"web_search","tool_args":"{\"query\":\"TSLA stock news\"}","tool_type":"builtin","tips":"Searched the web","iteration":1,"is_thinking":true,"outputs":{"query":"TSLA stock news","references":[{"index":1,"title":"...","url":"..."}]}}"#;
        let payload: NodeToolUseFinishedPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.tool_use_id, "call_abc123");
        assert_eq!(payload.status, "succeeded");
        assert_eq!(payload.tool_func_name, "web_search");
        assert!(payload.is_thinking);
        assert_eq!(payload.outputs.query.as_deref(), Some("TSLA stock news"));
        assert_eq!(payload.outputs.references.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn deserialize_plan_changed_payload_picks_up_sibling_tool_name() {
        let mut payload: PlanChangedPayload =
            serde_json::from_str(r#"{"node_id":"n_plan","started_at":1752048000}"#).unwrap();
        // `tool_name` lives outside `data` in the raw envelope; simulated
        // here the same way `map_conversation_event` fills it in.
        payload.tool_name = "planner".to_string();
        assert_eq!(payload.node_id, "n_plan");
        assert_eq!(payload.tool_name, "planner");
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
