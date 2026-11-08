package com.longbridge.agent;

/**
 * A context-compression pass has started, marking the start of a
 * context-compression pass triggered by a long conversation.
 */
public final class ContextCompressStartedEvent extends ConversationStreamEvent {
    private String startedAt;
    private String inputs;

    /**
     * Returns the start time, as an RFC 3339 timestamp. Unlike other events,
     * the timestamp here is a string rather than a Unix timestamp.
     *
     * @return start time (RFC 3339)
     */
    public String getStartedAt() {
        return startedAt;
    }

    /**
     * Returns the compression input summary, as JSON text.
     *
     * @return compression input summary (JSON text), or {@code null}
     */
    public String getInputs() {
        return inputs;
    }

    @Override
    public String toString() {
        return "ContextCompressStartedEvent [startedAt=" + startedAt + ", inputs=" + inputs + "]";
    }
}
