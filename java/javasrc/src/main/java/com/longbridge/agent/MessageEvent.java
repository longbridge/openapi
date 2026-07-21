package com.longbridge.agent;

/**
 * An incremental piece of the answer. This is the highest-frequency event;
 * concatenate {@link #getText} fragments in arrival order.
 */
public final class MessageEvent extends ConversationStreamEvent {
    private String text;
    private String messageType;
    private String key;
    private long startedAt;
    private String stage;
    private String stageTitle;
    private String stageFinishedTitle;
    private String outputs;

    /**
     * Returns the incremental text fragment.
     *
     * @return incremental text fragment
     */
    public String getText() {
        return text;
    }

    /**
     * Returns the fragment kind: {@code answer} — final answer text;
     * {@code think} — reasoning process; {@code process} — stage progress
     * description.
     *
     * @return fragment kind
     */
    public String getMessageType() {
        return messageType;
    }

    /**
     * Returns the identifier of the stream segment this fragment belongs to.
     * Fragments with the same key form one continuous block — group by key
     * when rendering.
     *
     * @return identifier of the stream segment this fragment belongs to
     */
    public String getKey() {
        return key;
    }

    /**
     * Returns the time this segment started, Unix timestamp in seconds.
     *
     * @return time this segment started
     */
    public long getStartedAt() {
        return startedAt;
    }

    /**
     * Returns the stage identifier; only present when {@link #getMessageType}
     * is {@code "process"}.
     *
     * @return stage identifier
     */
    public String getStage() {
        return stage;
    }

    /**
     * Returns the stage title while running; only present when
     * {@link #getMessageType} is {@code "process"}.
     *
     * @return stage title while running
     */
    public String getStageTitle() {
        return stageTitle;
    }

    /**
     * Returns the stage title after it finishes; only present when
     * {@link #getMessageType} is {@code "process"}.
     *
     * @return stage title after it finishes
     */
    public String getStageFinishedTitle() {
        return stageFinishedTitle;
    }

    /**
     * Returns the extra payload attached to the fragment, as JSON text;
     * usually absent.
     *
     * @return extra payload attached to the fragment (JSON text), or
     *         {@code null}
     */
    public String getOutputs() {
        return outputs;
    }

    @Override
    public String toString() {
        return "MessageEvent [text=" + text + ", messageType=" + messageType + ", key=" + key + ", startedAt="
                + startedAt + ", stage=" + stage + ", stageTitle=" + stageTitle + ", stageFinishedTitle="
                + stageFinishedTitle + ", outputs=" + outputs + "]";
    }
}
