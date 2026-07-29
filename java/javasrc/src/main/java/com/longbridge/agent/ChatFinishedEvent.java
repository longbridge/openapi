package com.longbridge.agent;

/**
 * Observed once all {@link MessageEvent}s for this round have been sent,
 * shortly before a {@link WorkflowFinishedEvent}.
 */
public final class ChatFinishedEvent extends ConversationStreamEvent {
    private long chatId;
    private String chatUid;
    private String messageId;
    private String error;
    private String errorMessage;

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
     * Returns the message ID of this round.
     *
     * @return message ID
     */
    public String getMessageId() {
        return messageId;
    }

    /**
     * Returns the error code; empty string in every run observed so far.
     *
     * @return error code
     */
    public String getError() {
        return error;
    }

    /**
     * Returns the error message; empty string in every run observed so far.
     *
     * @return error message
     */
    public String getErrorMessage() {
        return errorMessage;
    }

    @Override
    public String toString() {
        return "ChatFinishedEvent [chatId=" + chatId + ", chatUid=" + chatUid + ", messageId=" + messageId
                + ", error=" + error + ", errorMessage=" + errorMessage + "]";
    }
}
