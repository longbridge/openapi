package com.longbridge.quote;

import java.time.OffsetDateTime;

/**
 * Filing item
 */
public class FilingItem {
    private String id;
    private String title;
    private String description;
    private String fileName;
    private String[] fileUrls;
    private OffsetDateTime publishedAt;

    /**
     * Returns the filing ID.
     *
     * @return the filing ID
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
     * Returns the file name.
     *
     * @return the file name
     */
    public String getFileName() {
        return fileName;
    }

    /**
     * Returns the file URLs.
     *
     * @return the file URLs
     */
    public String[] getFileUrls() {
        return fileUrls;
    }

    /**
     * Returns the published time.
     *
     * @return the published time
     */
    public OffsetDateTime getPublishedAt() {
        return publishedAt;
    }

    @Override
    public String toString() {
        return "FilingItem [id=" + id + ", title=" + title + ", fileName=" + fileName + "]";
    }
}
