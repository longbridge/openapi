package com.longbridge.agent;

/**
 * The context-compression pass has finished.
 */
public final class ContextCompressFinishedEvent extends ConversationStreamEvent {
    private String createdAt;
    private String inputs;
    private String outputs;

    /**
     * Returns the finish time, as an RFC 3339 timestamp. Unlike other events,
     * the timestamp here is a string rather than a Unix timestamp.
     *
     * @return finish time (RFC 3339)
     */
    public String getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the compression input summary, as JSON text.
     *
     * @return compression input summary (JSON text), or {@code null}
     */
    public String getInputs() {
        return inputs;
    }

    /**
     * Returns the compression result summary, as JSON text.
     *
     * @return compression result summary (JSON text), or {@code null}
     */
    public String getOutputs() {
        return outputs;
    }

    @Override
    public String toString() {
        return "ContextCompressFinishedEvent [createdAt=" + createdAt + ", inputs=" + inputs + ", outputs=" + outputs
                + "]";
    }
}
