package com.longbridge.agent;

/**
 * An incremental piece of the answer.
 */
public final class MessageEvent extends ConversationStreamEvent {
    private String text;

    /**
     * Returns the incremental answer text.
     *
     * @return incremental answer text
     */
    public String getText() {
        return text;
    }

    @Override
    public String toString() {
        return "MessageEvent [text=" + text + "]";
    }
}
