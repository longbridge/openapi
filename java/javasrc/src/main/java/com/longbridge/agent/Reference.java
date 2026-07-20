package com.longbridge.agent;

/**
 * A source referenced by the answer
 */
public class Reference {
    private int index;
    private String title;
    private String url;

    /**
     * Returns the reference index.
     *
     * @return reference index
     */
    public int getIndex() {
        return index;
    }

    /**
     * Returns the reference title.
     *
     * @return reference title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the reference URL.
     *
     * @return reference URL
     */
    public String getUrl() {
        return url;
    }

    @Override
    public String toString() {
        return "Reference [index=" + index + ", title=" + title + ", url=" + url + "]";
    }
}
