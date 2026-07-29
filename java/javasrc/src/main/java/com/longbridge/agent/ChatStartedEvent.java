package com.longbridge.agent;

/**
 * The run has started. Always the first event of a stream.
 */
public final class ChatStartedEvent extends ConversationStreamEvent {
    private String chatUid;
    private String messageId;

    /**
     * Returns the conversation identifier.
     *
     * @return conversation identifier
     */
    public String getChatUid() {
        return chatUid;
    }

    /**
     * Returns the message ID of this round.
     *
     * @return message ID
     */
    public String getMessageId() {
        return messageId;
    }

    @Override
    public String toString() {
        return "ChatStartedEvent [chatUid=" + chatUid + ", messageId=" + messageId + "]";
    }
}
