package com.longbridge.agent;

import java.util.Arrays;

/**
 * Response for {@link AgentContext#chat}
 */
public class ChatDetail {
    private ChatInfo chat;
    private String chatRelation;
    private ChatMessage[] messages;

    /**
     * Returns the chat summary.
     *
     * @return chat summary
     */
    public ChatInfo getChat() {
        return chat;
    }

    /**
     * Returns the Agent / permission relation metadata, as a raw JSON string.
     *
     * @return chat relation JSON
     */
    public String getChatRelation() {
        return chatRelation;
    }

    /**
     * Returns the messages in the chat.
     *
     * @return messages
     */
    public ChatMessage[] getMessages() {
        return messages;
    }

    @Override
    public String toString() {
        return "ChatDetail [chat=" + chat + ", chatRelation=" + chatRelation + ", messages="
                + Arrays.toString(messages) + "]";
    }
}
