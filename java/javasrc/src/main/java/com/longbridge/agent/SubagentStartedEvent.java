package com.longbridge.agent;

import java.util.Arrays;

/**
 * The Agent has spawned a subagent to work on a sub-task. When the Agent
 * spawns a subagent, the subagent's lifecycle is reported with this
 * dedicated event family instead of {@code node_tool_use_*}.
 */
public final class SubagentStartedEvent extends ConversationStreamEvent {
    private String nodeId;
    private String toolUseId;
    private long startedAt;
    private String goal;
    private String prompt;
    private String subagentId;
    private String[] tools;

    /**
     * Returns the ID of the node that spawned the subagent.
     *
     * @return ID of the node that spawned the subagent
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the unique ID of this spawn; matches the finished event.
     *
     * @return unique ID of this spawn
     */
    public String getToolUseId() {
        return toolUseId;
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
     * Returns the goal assigned to the subagent.
     *
     * @return goal assigned to the subagent
     */
    public String getGoal() {
        return goal;
    }

    /**
     * Returns the full task prompt given to the subagent.
     *
     * @return full task prompt given to the subagent
     */
    public String getPrompt() {
        return prompt;
    }

    /**
     * Returns the subagent identifier; may be empty.
     *
     * @return subagent identifier
     */
    public String getSubagentId() {
        return subagentId;
    }

    /**
     * Returns the tools granted to the subagent, each as JSON text; may be
     * empty.
     *
     * @return tools granted to the subagent
     */
    public String[] getTools() {
        return tools;
    }

    @Override
    public String toString() {
        return "SubagentStartedEvent [nodeId=" + nodeId + ", toolUseId=" + toolUseId + ", startedAt=" + startedAt
                + ", goal=" + goal + ", prompt=" + prompt + ", subagentId=" + subagentId + ", tools="
                + Arrays.toString(tools) + "]";
    }
}
