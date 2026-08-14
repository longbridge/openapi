package com.longbridge.agent;

/**
 * One content chunk of a {@link ChatMessage}
 */
public class ChatMessageChunk {
    private String chunkType;
    private String content;
    private int index;
    private long startedAt;
    private long stoppedAt;

    /**
     * Returns the chunk type, e.g. {@code text}.
     *
     * @return chunk type
     */
    public String getChunkType() {
        return chunkType;
    }

    /**
     * Returns the chunk content.
     *
     * @return chunk content
     */
    public String getContent() {
        return content;
    }

    /**
     * Returns the index of the chunk within the message.
     *
     * @return chunk index
     */
    public int getIndex() {
        return index;
    }

    /**
     * Returns the start time, Unix timestamp in seconds.
     *
     * @return start time
     */
    public long getStartedAt() {
        return startedAt;
    }

    /**
     * Returns the stop time, Unix timestamp in seconds.
     *
     * @return stop time
     */
    public long getStoppedAt() {
        return stoppedAt;
    }

    @Override
    public String toString() {
        return "ChatMessageChunk [chunkType=" + chunkType + ", content=" + content + ", index=" + index
                + ", startedAt=" + startedAt + ", stoppedAt=" + stoppedAt + "]";
    }
}
