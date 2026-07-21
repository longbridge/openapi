package com.longbridge.agent;

import java.util.Arrays;

/**
 * The delegated Agent's run has finished.
 */
public final class AgentToolFinishedEvent extends ConversationStreamEvent {
    private String nodeId;
    private String toolUseId;
    private String agentToolName;
    private String status;
    private long startedAt;
    private double elapsedTime;
    private String error;
    private String toolArgs;
    private String outputs;
    private String toolType;
    private String tips;
    private String[] tipChips;
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
     * Returns the ID matching the {@code toolUseId} of
     * {@link AgentToolStartedEvent}.
     *
     * @return matching tool use ID
     */
    public String getToolUseId() {
        return toolUseId;
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
     * Returns the total duration in seconds.
     *
     * @return total duration in seconds
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
     * Returns the call arguments as a JSON string.
     *
     * @return call arguments (JSON string)
     */
    public String getToolArgs() {
        return toolArgs;
    }

    /**
     * Returns the result of the delegated Agent, as JSON text.
     *
     * @return result of the delegated Agent (JSON text), or {@code null}
     */
    public String getOutputs() {
        return outputs;
    }

    /**
     * Returns the tool category.
     *
     * @return tool category
     */
    public String getToolType() {
        return toolType;
    }

    /**
     * Returns the progress text; may be empty.
     *
     * @return progress text
     */
    public String getTips() {
        return tips;
    }

    /**
     * Returns the short tags; may be empty.
     *
     * @return short tags
     */
    public String[] getTipChips() {
        return tipChips;
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
        return "AgentToolFinishedEvent [nodeId=" + nodeId + ", toolUseId=" + toolUseId + ", agentToolName="
                + agentToolName + ", status=" + status + ", startedAt=" + startedAt + ", elapsedTime=" + elapsedTime
                + ", error=" + error + ", toolArgs=" + toolArgs + ", outputs=" + outputs + ", toolType=" + toolType
                + ", tips=" + tips + ", tipChips=" + Arrays.toString(tipChips) + ", isThinking=" + isThinking + "]";
    }
}
