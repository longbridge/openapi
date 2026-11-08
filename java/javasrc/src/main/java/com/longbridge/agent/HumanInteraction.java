package com.longbridge.agent;

import java.util.Arrays;

/**
 * A single interaction requested while an Agent workflow is paused
 */
public class HumanInteraction {
    private String toolCallId;
    private String interruptId;
    private String interactionType;
    private String toolName;
    private Question[] questions;
    private String toolArgs;

    /**
     * Returns the tool call that requested the interaction.
     *
     * @return tool call ID
     */
    public String getToolCallId() {
        return toolCallId;
    }

    /**
     * Returns the stable key expected by the answers map when continuing.
     *
     * @return interrupt ID
     */
    public String getInterruptId() {
        return interruptId;
    }

    /**
     * Returns the interaction type such as {@code ask_human} or
     * {@code trade_password}.
     *
     * @return interaction type
     */
    public String getInteractionType() {
        return interactionType;
    }

    /**
     * Returns the human-readable tool name.
     *
     * @return tool name
     */
    public String getToolName() {
        return toolName;
    }

    /**
     * Returns the questions and answer options presented to the user.
     *
     * @return questions
     */
    public Question[] getQuestions() {
        return questions;
    }

    /**
     * Returns the original tool arguments as a JSON string; empty when absent.
     *
     * @return tool arguments JSON
     */
    public String getToolArgs() {
        return toolArgs;
    }

    @Override
    public String toString() {
        return "HumanInteraction [toolCallId=" + toolCallId + ", interruptId=" + interruptId
                + ", interactionType=" + interactionType + ", toolName=" + toolName + ", questions="
                + Arrays.toString(questions) + ", toolArgs=" + toolArgs + "]";
    }
}
