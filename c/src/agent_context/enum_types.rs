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
    /// Observed once all `Message` events for this round have been sent;
    /// `chat_finished` is non-null
    ChatFinished,
    /// The run finished (succeeded, interrupted, failed, or stopped);
    /// `workflow_finished` is non-null
    WorkflowFinished,
    /// The server auto-generating a short title for the conversation;
    /// `chat_title_updated` is non-null
    ChatTitleUpdated,
    /// An event type not recognized by this SDK version; `other_json` is
    /// non-null and contains the raw event JSON
    Other,
}
