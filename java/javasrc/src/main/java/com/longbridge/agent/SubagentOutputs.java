package com.longbridge.agent;

import java.util.Arrays;

/**
 * The {@code outputs} sub-object of a {@link SubagentFinishedEvent}.
 */
public class SubagentOutputs {
    private String goal;
    private String result;
    private String[] subagentTools;

    /**
     * Returns the goal that was assigned to the subagent.
     *
     * @return the goal that was assigned to the subagent, or {@code null}
     */
    public String getGoal() {
        return goal;
    }

    /**
     * Returns the subagent's result.
     *
     * @return the subagent's result, or {@code null}
     */
    public String getResult() {
        return result;
    }

    /**
     * Returns the timeline of tool calls the subagent made, each as JSON
     * text.
     *
     * @return timeline of tool calls the subagent made
     */
    public String[] getSubagentTools() {
        return subagentTools;
    }

    @Override
    public String toString() {
        return "SubagentOutputs [goal=" + goal + ", result=" + result + ", subagentTools="
                + Arrays.toString(subagentTools) + "]";
    }
}
