package com.longbridge.agent;

import java.util.Arrays;

/**
 * The {@code outputs} sub-object of a {@link NodeToolUseFinishedEvent} —
 * only carries fields meant for display.
 */
public class NodeToolUseOutputs {
    private Reference[] references;
    private String[] referenceDomains;
    private String query;
    private String text;
    private String toolArgs;
    private String data;

    /**
     * Returns the sources referenced by the tool result.
     *
     * @return referenced sources
     */
    public Reference[] getReferences() {
        return references;
    }

    /**
     * Returns the domains of the referenced sources.
     *
     * @return domains of the referenced sources
     */
    public String[] getReferenceDomains() {
        return referenceDomains;
    }

    /**
     * Returns the query the tool executed.
     *
     * @return the query the tool executed, or {@code null}
     */
    public String getQuery() {
        return query;
    }

    /**
     * Returns the raw response text of the tool.
     *
     * @return raw response text of the tool, or {@code null}
     */
    public String getText() {
        return text;
    }

    /**
     * Returns the parsed request arguments, as JSON text.
     *
     * @return parsed request arguments (JSON text), or {@code null}
     */
    public String getToolArgs() {
        return toolArgs;
    }

    /**
     * Returns the structured result, as JSON text; present only for selected
     * tools.
     *
     * @return structured result (JSON text), or {@code null}
     */
    public String getData() {
        return data;
    }

    @Override
    public String toString() {
        return "NodeToolUseOutputs [references=" + Arrays.toString(references) + ", referenceDomains="
                + Arrays.toString(referenceDomains) + ", query=" + query + ", text=" + text + ", toolArgs=" + toolArgs
                + ", data=" + data + "]";
    }
}
