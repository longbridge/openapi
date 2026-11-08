package com.longbridge.agent;

/**
 * The server auto-generating a short title for the conversation as a UI
 * convenience. Can arrive before <em>or</em> after a
 * {@link WorkflowFinishedEvent}; not tied to the run's outcome.
 */
public final class ChatTitleUpdatedEvent extends ConversationStreamEvent {
    private long chatId;
    private String chatUid;
    private String source;
    private String title;
    private long updatedAt;

    /**
     * Returns the ID of the owning conversation.
     *
     * @return conversation ID
     */
    public long getChatId() {
        return chatId;
    }

    /**
     * Returns the conversation identifier.
     *
     * @return conversation identifier
     */
    public String getChatUid() {
        return chatUid;
    }

    /**
     * Returns where the title came from, e.g. {@code "ai_generated"}.
     *
     * @return where the title came from
     */
    public String getSource() {
        return source;
    }

    /**
     * Returns the new (possibly truncated) title.
     *
     * @return the new (possibly truncated) title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the Unix timestamp (in seconds) at which the title was updated.
     *
     * @return Unix timestamp (in seconds) at which the title was updated
     */
    public long getUpdatedAt() {
        return updatedAt;
    }

    @Override
    public String toString() {
        return "ChatTitleUpdatedEvent [chatId=" + chatId + ", chatUid=" + chatUid + ", source=" + source + ", title="
                + title + ", updatedAt=" + updatedAt + "]";
    }
}
