package com.longbridge.agent;

/**
 * The {@code inputs} sub-object of a {@link WorkflowStartedEvent}, echoing
 * the run's inputs.
 */
public class WorkflowStartedInputs {
    private long chatId;
    private String chatUid;
    private String messageId;
    private String query;

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
     * Returns the question that was asked.
     *
     * @return the question that was asked
     */
    public String getQuery() {
        return query;
    }

    @Override
    public String toString() {
        return "WorkflowStartedInputs [chatId=" + chatId + ", chatUid=" + chatUid + ", messageId=" + messageId
                + ", query=" + query + "]";
    }
}
