package com.longbridge.agent;

/**
 * The run finished (succeeded, interrupted, failed, or stopped), carrying the
 * run's outcome. Not necessarily the last event of the stream — the server
 * may still emit a few more housekeeping events (as an {@link OtherEvent},
 * e.g. a {@code chat_title_updated}) before actually closing the connection
 * (unless the stream itself errors first, delivered instead via
 * {@code Flow.Subscriber#onError}).
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
