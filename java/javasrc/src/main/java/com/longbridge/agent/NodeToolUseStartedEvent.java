package com.longbridge.agent;

import java.util.Arrays;

/**
 * An ordinary tool call has started. Match it to its
 * {@link NodeToolUseFinishedEvent} counterpart by {@link #getToolUseId}.
 */
public final class NodeToolUseStartedEvent extends ConversationStreamEvent {
    private String toolUseId;
    private String toolName;
    private String toolFuncName;
    private String toolArgs;
    private String tips;
    private String[] tipChips;
    private int iteration;
    private long startedAt;

    /**
     * Returns the unique ID of this call; matches the finished event.
     *
     * @return unique ID of this call
     */
    public String getToolUseId() {
        return toolUseId;
    }

    /**
     * Returns the localized display name of the tool.
     *
     * @return localized display name of the tool
     */
    public String getToolName() {
        return toolName;
    }

    /**
     * Returns the locale-stable tool identifier; use this for logic keyed on
     * the tool kind.
     *
     * @return locale-stable tool identifier
     */
    public String getToolFuncName() {
        return toolFuncName;
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
     * Returns progress text suitable for direct display, e.g.
     * {@code "Searching the web..."}.
     *
     * @return progress text
     */
    public String getTips() {
        return tips;
    }

    /**
     * Returns the short tags accompanying {@link #getTips}; may be empty.
     *
     * @return short tags accompanying the tips
     */
    public String[] getTipChips() {
        return tipChips;
    }

    /**
     * Returns the round number. Calls in the same round (same
     * {@link #getIteration}) run in parallel.
     *
     * @return round number
     */
    public int getIteration() {
        return iteration;
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
        return "NodeToolUseStartedEvent [toolUseId=" + toolUseId + ", toolName=" + toolName + ", toolFuncName="
                + toolFuncName + ", toolArgs=" + toolArgs + ", tips=" + tips + ", tipChips="
                + Arrays.toString(tipChips) + ", iteration=" + iteration + ", startedAt=" + startedAt + "]";
    }
}
