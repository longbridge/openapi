package com.longbridge.agent;

/**
 * Final run status of a conversation
 */
public enum ConversationStatus {
    /** The run completed successfully */
    Succeeded,
    /** The run is paused, waiting for {@link AgentContext#continueConversation} */
    Interrupted,
    /** The run failed */
    Failed,
    /** The run was stopped */
    Stopped,
}
