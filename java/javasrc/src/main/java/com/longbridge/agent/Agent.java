package com.longbridge.agent;

/**
 * An Agent in a Workspace
 */
public class Agent {
    private String uid;
    private String name;
    private String description;
    private String mode;
    private String icon;
    private boolean isPublished;
    private long publishedAt;
    private long createdAt;
    private long updatedAt;

    /**
     * Returns the Agent UID, used as the path parameter of
     * {@link AgentContext#conversation}.
     *
     * @return Agent UID
     */
    public String getUid() {
        return uid;
    }

    /**
     * Returns the Agent name.
     *
     * @return Agent name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the Agent description.
     *
     * @return Agent description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the Agent mode, e.g. {@code chat}.
     *
     * @return Agent mode
     */
    public String getMode() {
        return mode;
    }

    /**
     * Returns the icon URL.
     *
     * @return icon URL
     */
    public String getIcon() {
        return icon;
    }

    /**
     * Returns whether the Agent is published; only published Agents can start
     * conversations.
     *
     * @return {@code true} if published
     */
    public boolean isPublished() {
        return isPublished;
    }

    /**
     * Returns the publish time, Unix timestamp in seconds; {@code 0} if
     * unpublished.
     *
     * @return publish time
     */
    public long getPublishedAt() {
        return publishedAt;
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

    @Override
    public String toString() {
        return "Agent [uid=" + uid + ", name=" + name + ", description=" + description + ", mode=" + mode
                + ", icon=" + icon + ", isPublished=" + isPublished + ", publishedAt=" + publishedAt + ", createdAt="
                + createdAt + ", updatedAt=" + updatedAt + "]";
    }
}
