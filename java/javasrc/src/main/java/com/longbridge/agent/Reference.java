package com.longbridge.agent;

/**
 * A source referenced by the answer
 */
public class Reference {
    private int index;
    private int originalIndex;
    private String refType;
    private String id;
    private String title;
    private String url;
    private String content;

    /**
     * Returns the reference index.
     *
     * @return reference index
     */
    public int getIndex() {
        return index;
    }

    /**
     * Returns the original index in the source list, before any reranking.
     *
     * @return original reference index
     */
    public int getOriginalIndex() {
        return originalIndex;
    }

    /**
     * Returns the reference kind, e.g. {@code "NewsArticle"}.
     *
     * @return reference kind
     */
    public String getRefType() {
        return refType;
    }

    /**
     * Returns the reference id.
     *
     * @return reference id
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the reference title. Often empty at the top level — the
     * human-readable title usually lives in {@link #getContent}.
     *
     * @return reference title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the reference URL. Often empty at the top level — see
     * {@link #getContent}.
     *
     * @return reference URL
     */
    public String getUrl() {
        return url;
    }

    /**
     * Returns the full reference payload as sent by the server ({@code
     * source}, {@code description}, {@code published_at}, {@code
     * source_url}, {@code source_logo}, {@code kind}, …), as JSON text. Kept
     * as raw JSON because the field set varies by {@link #getRefType}.
     *
     * @return full reference payload (JSON text), or {@code null}
     */
    public String getContent() {
        return content;
    }

    @Override
    public String toString() {
        return "Reference [index=" + index + ", originalIndex=" + originalIndex + ", refType=" + refType + ", id="
                + id + ", title=" + title + ", url=" + url + ", content=" + content + "]";
    }
}
