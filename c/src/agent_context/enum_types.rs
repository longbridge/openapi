use longbridge_c_macros::CEnum;

/// Final run status of a conversation
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::agent::ConversationStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CConversationStatus {
    /// The run completed successfully
    #[c(remote = "Succeeded")]
    ConversationStatusSucceeded,
    /// The run is paused, waiting for
    /// `lb_agent_context_continue_conversation`/
    /// `lb_agent_context_continue_conversation_streamed`
    #[c(remote = "Interrupted")]
    ConversationStatusInterrupted,
    /// The run failed
    #[c(remote = "Failed")]
    ConversationStatusFailed,
    /// The run was stopped
    #[c(remote = "Stopped")]
    ConversationStatusStopped,
}

/// Kind of a [`crate::agent_context::types::CConversationStreamEvent`]. Only
/// the field matching this kind is non-null, all others are null.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub enum CConversationStreamEventType {
    /// The run has started; `chat_started` is non-null
    ChatStarted,
    /// Observed right after `ChatStarted` on every run seen so far;
    /// `workflow_started` is non-null
    WorkflowStarted,
    /// An incremental piece of the answer; `message` is non-null
    Message,
    /// A heartbeat with no payload, observed at arbitrary points in the
    /// stream (including in between `Message` chunks); every field below is
    /// null
    Ping,
    /// The Agent has entered the reasoning phase; `thinking_started` is
    /// non-null
    ThinkingStarted,
    /// The reasoning phase is over; `thinking_finished` is non-null
    ThinkingFinished,
    /// An ordinary tool call has started; `node_tool_use_started` is
    /// non-null
    NodeToolUseStarted,
    /// An ordinary tool call has ended; `node_tool_use_finished` is
    /// non-null
    NodeToolUseFinished,
    /// The Agent has spawned a subagent to work on a sub-task;
    /// `subagent_started` is non-null
    SubagentStarted,
    /// The subagent has called one of its own tools; `subagent_progress` is
    /// non-null
    SubagentProgress,
    /// The subagent has finished its sub-task; `subagent_finished` is
    /// non-null
    SubagentFinished,
    /// The Agent has delegated to another Agent as a tool;
    /// `agent_tool_started` is non-null
    AgentToolStarted,
    /// The delegated Agent has called one of its own tools;
    /// `agent_tool_progress` is non-null
    AgentToolProgress,
    /// The delegated Agent's run has finished; `agent_tool_finished` is
    /// non-null
    AgentToolFinished,
    /// The run is paused: the Agent needs more information or confirmation
    /// from you; `human_interaction_required` is non-null. Unlike
    /// `WorkflowFinished`, this is emitted instead of (never alongside)
    /// `WorkflowFinished` for the same run
    HumanInteractionRequired,
    /// Sensitive content in the user query was masked before processing;
    /// `query_masked` is non-null
    QueryMasked,
    /// The Agent created or updated its task plan; `plan_changed` is
    /// non-null
    PlanChanged,
    /// A context-compression pass has started; `context_compress_started`
    /// is non-null
    ContextCompressStarted,
    /// The context-compression pass has finished;
    /// `context_compress_finished` is non-null
    ContextCompressFinished,
    /// Observed once all `Message` events for this round have been sent;
    /// `chat_finished` is non-null
    ChatFinished,
    /// The run finished successfully, with a failure, or stopped by the
    /// user; `workflow_finished` is non-null. Never emitted for an
    /// interrupted run — see `HumanInteractionRequired` for that case
    WorkflowFinished,
    /// The server auto-generating a short title for the conversation;
    /// `chat_title_updated` is non-null
    ChatTitleUpdated,
    /// An event type not recognized by this SDK version; `other_json` is
    /// non-null and contains the raw event JSON
    Other,
}
