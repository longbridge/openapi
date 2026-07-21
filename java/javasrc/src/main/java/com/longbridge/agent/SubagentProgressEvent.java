package com.longbridge.agent;

/**
 * Emitted every time the subagent calls one of its own tools. Use it to
 * render a live timeline inside the subagent card.
 */
public final class SubagentProgressEvent extends ConversationStreamEvent {
    private String nodeId;
    private String parentToolCallId;
    private String subagentToolName;
    private String subagentToolArgs;
    private String subagentStatus;
    private long subagentDurationMs;
    private int subagentIteration;
    private long startedAt;

    /**
     * Returns the ID of the node that spawned the subagent.
     *
     * @return ID of the node that spawned the subagent
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the {@code toolUseId} of the owning {@link SubagentStartedEvent}.
     *
     * @return {@code toolUseId} of the owning {@link SubagentStartedEvent}
     */
    public String getParentToolCallId() {
        return parentToolCallId;
    }

    /**
     * Returns the name of the tool the subagent called.
     *
     * @return name of the tool the subagent called
     */
    public String getSubagentToolName() {
        return subagentToolName;
    }

    /**
     * Returns the arguments of that call, as a JSON string.
     *
     * @return arguments of that call (JSON string)
     */
    public String getSubagentToolArgs() {
        return subagentToolArgs;
    }

    /**
     * Returns the status of that call: {@code running} / {@code succeeded} /
     * {@code failed}.
     *
     * @return status of that call
     */
    public String getSubagentStatus() {
        return subagentStatus;
    }

    /**
     * Returns the duration of that call in milliseconds.
     *
     * @return duration of that call in milliseconds
     */
    public long getSubagentDurationMs() {
        return subagentDurationMs;
    }

    /**
     * Returns the subagent's internal round number.
     *
     * @return subagent's internal round number
     */
    public int getSubagentIteration() {
        return subagentIteration;
    }

    /**
     * Returns the start time, Unix timestamp in seconds.
     *
     * @return start time
     */
    public long getStartedAt() {
        return startedAt;
    }

    @Override
    public String toString() {
        return "SubagentProgressEvent [nodeId=" + nodeId + ", parentToolCallId=" + parentToolCallId
                + ", subagentToolName=" + subagentToolName + ", subagentToolArgs=" + subagentToolArgs
                + ", subagentStatus=" + subagentStatus + ", subagentDurationMs=" + subagentDurationMs
                + ", subagentIteration=" + subagentIteration + ", startedAt=" + startedAt + "]";
    }
}
