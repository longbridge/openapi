package com.longbridge.fund;

/**
 * FundOrderKeyword
 */
public class FundOrderKeyword {
    private String content;
    private String group;
    private String key;
    private String lineStrategy;
    private String title;

    /**
     * Returns content.
     *
     * @return content
     */
    public String getContent() {
        return content;
    }

    /**
     * Returns group.
     *
     * @return group
     */
    public String getGroup() {
        return group;
    }

    /**
     * Returns key.
     *
     * @return key
     */
    public String getKey() {
        return key;
    }

    /**
     * Returns lineStrategy.
     *
     * @return lineStrategy
     */
    public String getLineStrategy() {
        return lineStrategy;
    }

    /**
     * Returns title.
     *
     * @return title
     */
    public String getTitle() {
        return title;
    }

    @Override
    public String toString() {
        return                 "FundOrderKeyword [content=" + content +
                ", group=" + group +
                ", key=" + key +
                ", lineStrategy=" + lineStrategy +
                ", title=" + title + "]";
    }
}
