package com.longbridge.agent;

/**
 * A Workspace the current account belongs to
 */
public class Workspace {
    private String id;
    private String name;
    private long createdAt;
    private long updatedAt;

    /**
     * Returns the Workspace ID.
     *
     * @return Workspace ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the Workspace name.
     *
     * @return Workspace name
     */
    public String getName() {
        return name;
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
        return "Workspace [id=" + id + ", name=" + name + ", createdAt=" + createdAt + ", updatedAt=" + updatedAt
                + "]";
    }
}
