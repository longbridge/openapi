package com.longbridge.agent;

import java.util.Arrays;

/**
 * The Agent has delegated to another Agent as a tool. When the Agent
 * delegates to another Agent as a tool, that inner run is reported with the
 * {@code agent_tool_*} family — the shape mirrors the subagent events.
 */
public final class AgentToolStartedEvent extends ConversationStreamEvent {
    private String nodeId;
    private String toolUseId;
    private String agentToolName;
    private String title;
    private long startedAt;
    private String toolArgs;
    private String toolName;
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
     * Returns the unique ID of this call; matches the finished event.
     *
     * @return unique ID of this call
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
     * Returns the display title; may be empty.
     *
     * @return display title
     */
    public String getTitle() {
        return title;
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
     * Returns the call arguments as a JSON string.
     *
     * @return call arguments (JSON string)
     */
    public String getToolArgs() {
        return toolArgs;
    }

    /**
     * Returns the localized display name.
     *
     * @return localized display name
     */
    public String getToolName() {
        return toolName;
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
     * @return {@code true} if called during the thinking phase
     */
    public boolean isThinking() {
        return isThinking;
    }

    @Override
    public String toString() {
        return "AgentToolStartedEvent [nodeId=" + nodeId + ", toolUseId=" + toolUseId + ", agentToolName="
                + agentToolName + ", title=" + title + ", startedAt=" + startedAt + ", toolArgs=" + toolArgs
                + ", toolName=" + toolName + ", tips=" + tips + ", tipChips=" + Arrays.toString(tipChips)
                + ", isThinking=" + isThinking + "]";
    }
}
