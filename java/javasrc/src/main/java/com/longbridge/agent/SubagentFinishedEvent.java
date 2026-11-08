package com.longbridge.agent;

/**
 * The subagent has finished its sub-task.
 */
public final class SubagentFinishedEvent extends ConversationStreamEvent {
    private String nodeId;
    private String toolUseId;
    private String status;
    private long startedAt;
    private double elapsedTime;
    private String error;
    private SubagentOutputs outputs;

    /**
     * Returns the ID of the node that spawned the subagent.
     *
     * @return ID of the node that spawned the subagent
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the ID matching the {@code toolUseId} of
     * {@link SubagentStartedEvent}.
     *
     * @return matching tool use ID
     */
    public String getToolUseId() {
        return toolUseId;
    }

    /**
     * Returns the status: {@code succeeded} / {@code failed}.
     *
     * @return status
     */
    public String getStatus() {
        return status;
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
     * Returns the total subagent duration in seconds.
     *
     * @return total subagent duration in seconds
     */
    public double getElapsedTime() {
        return elapsedTime;
    }

    /**
     * Returns the error description on failure.
     *
     * @return error description
     */
    public String getError() {
        return error;
    }

    /**
     * Returns the subagent result: goal, result, and the timeline of tool
     * calls it made.
     *
     * @return subagent result
     */
    public SubagentOutputs getOutputs() {
        return outputs;
    }

    @Override
    public String toString() {
        return "SubagentFinishedEvent [nodeId=" + nodeId + ", toolUseId=" + toolUseId + ", status=" + status
                + ", startedAt=" + startedAt + ", elapsedTime=" + elapsedTime + ", error=" + error + ", outputs="
                + outputs + "]";
    }
}
