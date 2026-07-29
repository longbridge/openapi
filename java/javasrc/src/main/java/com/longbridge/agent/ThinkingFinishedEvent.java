package com.longbridge.agent;

/**
 * The reasoning phase is over; answer text ({@link MessageEvent} with
 * {@code messageType == "answer"}) follows.
 */
public final class ThinkingFinishedEvent extends ConversationStreamEvent {
    private long finishedAt;
    private int elapsedTime;

    /**
     * Returns the finish time, Unix timestamp in seconds.
     *
     * @return finish time
     */
    public long getFinishedAt() {
        return finishedAt;
    }

    /**
     * Returns the reasoning duration in seconds.
     *
     * @return reasoning duration in seconds
     */
    public int getElapsedTime() {
        return elapsedTime;
    }

    @Override
    public String toString() {
        return "ThinkingFinishedEvent [finishedAt=" + finishedAt + ", elapsedTime=" + elapsedTime + "]";
    }
}
