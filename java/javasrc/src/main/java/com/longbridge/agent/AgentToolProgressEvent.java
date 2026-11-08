package com.longbridge.agent;

/**
 * Emitted for each inner tool call the delegated Agent makes.
 */
public final class AgentToolProgressEvent extends ConversationStreamEvent {
    private String nodeId;
    private String parentToolCallId;
    private String agentToolName;
    private String innerToolName;
    private String innerToolArgs;
    private String status;
    private long durationMs;
    private long startedAt;
    private boolean isThinking;

    /**
     * Returns the ID of the calling node.
     *
     * @return ID of the calling node
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the {@code toolUseId} of the owning
     * {@link AgentToolStartedEvent}.
     *
     * @return {@code toolUseId} of the owning {@link AgentToolStartedEvent}
     */
    public String getParentToolCallId() {
        return parentToolCallId;
    }

    /**
     * Returns the identifier of the Agent being called.
     *
     * @return identifier of the Agent being called
     */
    public String getAgentToolName() {
        return agentToolName;
    }

    /**
     * Returns the name of the inner tool the delegated Agent called.
     *
     * @return name of the inner tool the delegated Agent called
     */
    public String getInnerToolName() {
        return innerToolName;
    }

    /**
     * Returns the arguments of that inner call, as a JSON string.
     *
     * @return arguments of that inner call (JSON string)
     */
    public String getInnerToolArgs() {
        return innerToolArgs;
    }

    /**
     * Returns the status of the inner call: {@code running} /
     * {@code succeeded} / {@code failed}.
     *
     * @return status of the inner call
     */
    public String getStatus() {
        return status;
    }

    /**
     * Returns the duration of the inner call in milliseconds.
     *
     * @return duration of the inner call in milliseconds
     */
    public long getDurationMs() {
        return durationMs;
    }

    /**
     * Returns the start time, Unix timestamp in seconds.
     *
     * @return start time
     */
    public long getStartedAt() {
        return startedAt;
    }

    /**
     * Returns whether the call happened during the thinking phase.
     *
     * @return {@code true} if during the thinking phase
     */
    public boolean isThinking() {
        return isThinking;
    }

    @Override
    public String toString() {
        return "AgentToolProgressEvent [nodeId=" + nodeId + ", parentToolCallId=" + parentToolCallId
                + ", agentToolName=" + agentToolName + ", innerToolName=" + innerToolName + ", innerToolArgs="
                + innerToolArgs + ", status=" + status + ", durationMs=" + durationMs + ", startedAt=" + startedAt
                + ", isThinking=" + isThinking + "]";
    }
}
