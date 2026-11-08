package com.longbridge.agent;

/**
 * Observed right after a {@link ChatStartedEvent} on every run seen so far.
 */
public final class WorkflowStartedEvent extends ConversationStreamEvent {
    private boolean hitCache;
    private WorkflowStartedInputs inputs;
    private long startedAt;
    private long workflowId;

    /**
     * Returns whether this run's answer was served from a cache.
     *
     * @return {@code true} if this run's answer was served from a cache
     */
    public boolean isHitCache() {
        return hitCache;
    }

    /**
     * Returns the echoed inputs of the run.
     *
     * @return the echoed inputs of the run
     */
    public WorkflowStartedInputs getInputs() {
        return inputs;
    }

    /**
     * Returns the Unix timestamp (in seconds) at which the run started.
     *
     * @return Unix timestamp (in seconds) at which the run started
     */
    public long getStartedAt() {
        return startedAt;
    }

    /**
     * Returns the internal workflow run ID.
     *
     * @return internal workflow run ID
     */
    public long getWorkflowId() {
        return workflowId;
    }

    @Override
    public String toString() {
        return "WorkflowStartedEvent [hitCache=" + hitCache + ", inputs=" + inputs + ", startedAt=" + startedAt
                + ", workflowId=" + workflowId + "]";
    }
}
