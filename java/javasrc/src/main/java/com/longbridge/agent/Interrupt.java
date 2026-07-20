package com.longbridge.agent;

import java.util.Arrays;

/**
 * Present when a conversation run is interrupted, waiting for
 * {@link AgentContext#continueConversation}
 */
public class Interrupt {
    private String nodeId;
    private String toolCallId;
    private Question[] questions;
    private long messageId;
    private long chatId;

    /**
     * Returns the ID of the node that triggered the interrupt.
     *
     * @return node ID
     */
    public String getNodeId() {
        return nodeId;
    }

    /**
     * Returns the tool call ID of this inquiry; used as the answer key when
     * continuing.
     *
     * @return tool call ID
     */
    public String getToolCallId() {
        return toolCallId;
    }

    /**
     * Returns the questions you need to answer.
     *
     * @return questions
     */
    public Question[] getQuestions() {
        return questions;
    }

    /**
     * Returns the ID of the paused message.
     *
     * @return message ID
     */
    public long getMessageId() {
        return messageId;
    }

    /**
     * Returns the ID of the owning conversation.
     *
     * @return conversation ID
     */
    public long getChatId() {
        return chatId;
    }

    @Override
    public String toString() {
        return "Interrupt [nodeId=" + nodeId + ", toolCallId=" + toolCallId + ", questions="
                + Arrays.toString(questions) + ", messageId=" + messageId + ", chatId=" + chatId + "]";
    }
}
