package com.longbridge.agent;

/**
 * The Agent created or updated its task plan.
 */
public final class PlanChangedEvent extends ConversationStreamEvent {
    private String nodeId;
    private long startedAt;
    private String outputs;
    private String toolName;

    /**
     * Returns the ID of the planning node.
     *
     * @return ID of the planning node
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the time of the change, Unix timestamp in seconds.
     *
     * @return time of the change
     */
    public long getStartedAt() {
        return startedAt;
    }

    /**
     * Returns the current plan content, as JSON text.
     *
     * @return current plan content (JSON text), or {@code null}
     */
    public String getOutputs() {
        return outputs;
    }

    /**
     * Returns the identifier of the planning tool.
     *
     * @return identifier of the planning tool
     */
    public String getToolName() {
        return toolName;
    }

    @Override
    public String toString() {
        return "PlanChangedEvent [nodeId=" + nodeId + ", startedAt=" + startedAt + ", outputs=" + outputs
                + ", toolName=" + toolName + "]";
    }
}
