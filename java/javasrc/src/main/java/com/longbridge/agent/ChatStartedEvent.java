package com.longbridge.agent;

/**
 * The run has started. Always the first event of a stream.
 */
public final class ChatStartedEvent extends ConversationStreamEvent {
    private String chatUid;
    private String messageId;
    private long chatId;
    private String error;
    private String errorMessage;

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

    /**
     * Returns the ID of the owning conversation.
     *
     * @return owning conversation ID
     */
    public long getChatId() {
        return chatId;
    }

    /**
     * Returns the error detail; empty at start.
     *
     * @return error detail
     */
    public String getError() {
        return error;
    }

    /**
     * Returns the user-facing error message; empty at start.
     *
     * @return user-facing error message
     */
    public String getErrorMessage() {
        return errorMessage;
    }

    @Override
    public String toString() {
        return "ChatStartedEvent [chatUid=" + chatUid + ", messageId=" + messageId + ", chatId=" + chatId
                + ", error=" + error + ", errorMessage=" + errorMessage + "]";
    }
}
