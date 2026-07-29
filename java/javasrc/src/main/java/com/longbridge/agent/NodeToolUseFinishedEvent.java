package com.longbridge.agent;

import java.util.Arrays;

/**
 * The tool call has ended.
 */
public final class NodeToolUseFinishedEvent extends ConversationStreamEvent {
    private String toolUseId;
    private String status;
    private String error;
    private double elapsedTime;
    private long startedAt;
    private String toolName;
    private String toolFuncName;
    private String toolArgs;
    private String toolType;
    private String tips;
    private String[] tipChips;
    private int iteration;
    private boolean isThinking;
    private NodeToolUseOutputs outputs;

    /**
     * Returns the ID matching the {@code toolUseId} of the started event.
     *
     * @return matching tool use ID
     */
    public String getToolUseId() {
        return toolUseId;
    }

    /**
     * Returns the call status: {@code succeeded} / {@code failed}.
     *
     * @return call status
     */
    public String getStatus() {
        return status;
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
     * Returns the call duration in seconds.
     *
     * @return call duration in seconds
     */
    public double getElapsedTime() {
        return elapsedTime;
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
     * Returns the localized display name.
     *
     * @return localized display name
     */
    public String getToolName() {
        return toolName;
    }

    /**
     * Returns the locale-stable tool identifier.
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
     * Returns the tool category.
     *
     * @return tool category
     */
    public String getToolType() {
        return toolType;
    }

    /**
     * Returns the progress text.
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
     * Returns the round number.
     *
     * @return round number
     */
    public int getIteration() {
        return iteration;
    }

    /**
     * Returns whether the call happened during the thinking phase.
     *
     * @return {@code true} if the call happened during the thinking phase
     */
    public boolean isThinking() {
        return isThinking;
    }

    /**
     * Returns the filtered call results, for display.
     *
     * @return filtered call results
     */
    public NodeToolUseOutputs getOutputs() {
        return outputs;
    }

    @Override
    public String toString() {
        return "NodeToolUseFinishedEvent [toolUseId=" + toolUseId + ", status=" + status + ", error=" + error
                + ", elapsedTime=" + elapsedTime + ", startedAt=" + startedAt + ", toolName=" + toolName
                + ", toolFuncName=" + toolFuncName + ", toolArgs=" + toolArgs + ", toolType=" + toolType + ", tips="
                + tips + ", tipChips=" + Arrays.toString(tipChips) + ", iteration=" + iteration + ", isThinking="
                + isThinking + ", outputs=" + outputs + "]";
    }
}
