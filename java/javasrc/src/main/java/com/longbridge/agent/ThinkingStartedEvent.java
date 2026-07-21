package com.longbridge.agent;

/**
 * The Agent has entered the reasoning phase (analyzing the question, planning
 * tool calls). Between this and a {@link ThinkingFinishedEvent},
 * {@link MessageEvent}s with {@code messageType == "think"} and tool-call
 * events may arrive.
 */
public final class ThinkingStartedEvent extends ConversationStreamEvent {
    private long startedAt;

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
        return "ThinkingStartedEvent [startedAt=" + startedAt + "]";
    }
}
