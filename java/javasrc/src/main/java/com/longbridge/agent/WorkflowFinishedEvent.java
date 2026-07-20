package com.longbridge.agent;

/**
 * The run finished (succeeded, interrupted, failed, or stopped). Always the
 * last event of a stream (unless the stream itself errors first, delivered
 * instead via {@code Flow.Subscriber#onError}).
 */
public final class WorkflowFinishedEvent extends ConversationStreamEvent {
    private ConversationResponse response;

    /**
     * Returns the final conversation response, equivalent to what a blocking
     * call to {@link AgentContext#conversation} or
     * {@link AgentContext#continueConversation} would have returned.
     *
     * @return final conversation response
     */
    public ConversationResponse getResponse() {
        return response;
    }

    @Override
    public String toString() {
        return "WorkflowFinishedEvent [response=" + response + "]";
    }
}
