package com.longbridge.content;

import java.time.OffsetDateTime;

/**
 * Topic item
 */
public class TopicItem {
    private String id;
    private String title;
    private String description;
    private String url;
    private OffsetDateTime publishedAt;
    private int commentsCount;
    private int likesCount;
    private int sharesCount;

    /**
     * Returns the topic ID.
     *
     * @return the topic ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the title.
     *
     * @return the title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the description.
     *
     * @return the description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the URL.
     *
     * @return the URL
     */
    public String getUrl() {
        return url;
    }

    /**
     * Returns the published time.
     *
     * @return the published time
     */
    public OffsetDateTime getPublishedAt() {
        return publishedAt;
    }

    /**
     * Returns the comments count.
     *
     * @return the comments count
     */
    public int getCommentsCount() {
        return commentsCount;
    }

    /**
     * Returns the likes count.
     *
     * @return the likes count
     */
    public int getLikesCount() {
        return likesCount;
    }

    /**
     * Returns the shares count.
     *
     * @return the shares count
     */
    public int getSharesCount() {
        return sharesCount;
    }

    @Override
    public String toString() {
        return "TopicItem [id=" + id + ", title=" + title + ", description=" + description
                + ", url=" + url + ", publishedAt=" + publishedAt + ", commentsCount=" + commentsCount
                + ", likesCount=" + likesCount + ", sharesCount=" + sharesCount + "]";
    }
}
