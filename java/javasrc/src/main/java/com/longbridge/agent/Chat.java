package com.longbridge.agent;

/**
 * A chat (conversation) with an Agent
 */
public class Chat {
    private long id;
    private String uid;
    private String name;
    private long agentId;
    private String agentName;
    private String agentUid;
    private String fromSource;
    private boolean hasUnread;
    private long createdAt;
    private long updatedAt;
    private String chatRelation;

    /**
     * Returns the chat ID.
     *
     * @return chat ID
     */
    public long getId() {
        return id;
    }

    /**
     * Returns the chat UID, used as the path parameter of
     * {@link AgentContext#chat}.
     *
     * @return chat UID
     */
    public String getUid() {
        return uid;
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
     * Returns the ID of the Agent this chat belongs to.
     *
     * @return Agent ID
     */
    public long getAgentId() {
        return agentId;
    }

    /**
     * Returns the name of the Agent this chat belongs to.
     *
     * @return Agent name
     */
    public String getAgentName() {
        return agentName;
    }

    /**
     * Returns the UID of the Agent this chat belongs to.
     *
     * @return Agent UID
     */
    public String getAgentUid() {
        return agentUid;
    }

    /**
     * Returns the source the chat was created from, e.g. {@code api}.
     *
     * @return source
     */
    public String getFromSource() {
        return fromSource;
    }

    /**
     * Returns whether the chat has unread messages.
     *
     * @return whether the chat has unread messages
     */
    public boolean getHasUnread() {
        return hasUnread;
    }

    /**
     * Returns the creation time, Unix timestamp in seconds.
     *
     * @return creation time
     */
    public long getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the last updated time, Unix timestamp in seconds.
     *
     * @return last updated time
     */
    public long getUpdatedAt() {
        return updatedAt;
    }

    /**
     * Returns the Agent / permission relation metadata, as a raw JSON string.
     *
     * @return chat relation JSON
     */
    public String getChatRelation() {
        return chatRelation;
    }

    @Override
    public String toString() {
        return "Chat [id=" + id + ", uid=" + uid + ", name=" + name + ", agentId=" + agentId
                + ", agentName=" + agentName + ", agentUid=" + agentUid + ", fromSource=" + fromSource
                + ", hasUnread=" + hasUnread + ", createdAt=" + createdAt + ", updatedAt=" + updatedAt
                + ", chatRelation=" + chatRelation + "]";
    }
}
