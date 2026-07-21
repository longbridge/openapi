package com.longbridge.agent;

/**
 * The run is paused: the Agent needs more information or confirmation from
 * you, carrying the interrupt to resume from via
 * {@link AgentContext#continueConversation}/
 * {@link AgentContext#continueConversationStream}. Unlike
 * {@link WorkflowFinishedEvent}, this is emitted instead of (never alongside)
 * a {@link WorkflowFinishedEvent} for the same run — an interrupted run never
 * emits {@code workflow_finished} at all, so this is the terminal/final-result
 * event for that outcome instead.
 */
public final class HumanInteractionRequiredEvent extends ConversationStreamEvent {
    private ConversationResponse response;

    /**
     * Returns the final conversation response, equivalent to what a blocking
     * call to {@link AgentContext#conversation} or
     * {@link AgentContext#continueConversation} would have returned. Its
     * {@link ConversationResponse#getStatus} is
     * {@link ConversationStatus#Interrupted}, and
     * {@link ConversationResponse#getInterrupt} carries the questions to
     * answer.
     *
     * @return final conversation response
     */
    public ConversationResponse getResponse() {
        return response;
    }

    @Override
    public String toString() {
        return "HumanInteractionRequiredEvent [response=" + response + "]";
    }
}
