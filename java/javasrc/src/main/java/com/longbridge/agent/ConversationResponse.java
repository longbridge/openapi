package com.longbridge.agent;

import java.util.Arrays;

/**
 * Response for {@link AgentContext#conversation},
 * {@link AgentContext#continueConversation}, and the final result of the
 * streamed counterparts (delivered as {@link WorkflowFinishedEvent#getResponse}).
 */
public class ConversationResponse {
    private String chatUid;
    private String messageId;
    private ConversationStatus status;
    private String answer;
    private Reference[] references;
    private double elapsedTime;
    private Interrupt interrupt;
    private ConversationError error;

    /**
     * Returns the conversation identifier, used for follow-up questions and
     * troubleshooting.
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
     * Returns the final run status.
     *
     * @return final run status
     */
    public ConversationStatus getStatus() {
        return status;
    }

    /**
     * Returns the final answer text; valid when {@link #getStatus} is
     * {@link ConversationStatus#Succeeded}.
     *
     * @return final answer text
     */
    public String getAnswer() {
        return answer;
    }

    /**
     * Returns the sources referenced by the answer.
     *
     * @return referenced sources
     */
    public Reference[] getReferences() {
        return references;
    }

    /**
     * Returns the run duration in seconds.
     *
     * @return run duration in seconds
     */
    public double getElapsedTime() {
        return elapsedTime;
    }

    /**
     * Returns the interrupt details; present only when {@link #getStatus} is
     * {@link ConversationStatus#Interrupted}.
     *
     * @return interrupt details, or {@code null}
     */
    public Interrupt getInterrupt() {
        return interrupt;
    }

    /**
     * Returns the error details; present only when the run failed.
     *
     * @return error details, or {@code null}
     */
    public ConversationError getError() {
        return error;
    }

    @Override
    public String toString() {
        return "ConversationResponse [chatUid=" + chatUid + ", messageId=" + messageId + ", status=" + status
                + ", answer=" + answer + ", references=" + Arrays.toString(references) + ", elapsedTime="
                + elapsedTime + ", interrupt=" + interrupt + ", error=" + error + "]";
    }
}
