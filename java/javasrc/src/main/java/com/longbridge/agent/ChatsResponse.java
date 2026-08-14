package com.longbridge.agent;

import java.util.Arrays;

/**
 * Response for {@link AgentContext#chats}
 */
public class ChatsResponse {
    private Chat[] chats;

    /**
     * Returns the chat list.
     *
     * @return chat list
     */
    public Chat[] getChats() {
        return chats;
    }

    @Override
    public String toString() {
        return "ChatsResponse [chats=" + Arrays.toString(chats) + "]";
    }
}
