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
    /// An incremental piece of the answer; `message` is non-null
    Message,
    /// The run finished (succeeded, interrupted, failed, or stopped);
    /// `workflow_finished` is non-null
    WorkflowFinished,
    /// An event type not recognized by this SDK version; `other_json` is
    /// non-null and contains the raw event JSON
    Other,
}
