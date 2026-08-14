package com.longbridge.agent;

/**
 * Chat summary carried in the {@link ChatDetail} response
 */
public class ChatInfo {
    private long id;
    private String name;
    private String uid;

    /**
     * Returns the chat ID.
     *
     * @return chat ID
     */
    public long getId() {
        return id;
    }

    /**
     * Returns the chat name (title).
     *
     * @return chat name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the chat UID.
     *
     * @return chat UID
     */
    public String getUid() {
        return uid;
    }

    @Override
    public String toString() {
        return "ChatInfo [id=" + id + ", name=" + name + ", uid=" + uid + "]";
    }
}
