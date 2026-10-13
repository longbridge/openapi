package com.longbridge.fund;

/**
 * FundOrderStage
 */
public class FundOrderStage {
    private String desc;
    private String key;
    private String link;
    private String linkText;
    private String progress;
    private String stage;

    /**
     * Returns desc.
     *
     * @return desc
     */
    public String getDesc() {
        return desc;
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
     * Returns link.
     *
     * @return link
     */
    public String getLink() {
        return link;
    }

    /**
     * Returns linkText.
     *
     * @return linkText
     */
    public String getLinkText() {
        return linkText;
    }

    /**
     * Returns progress.
     *
     * @return progress
     */
    public String getProgress() {
        return progress;
    }

    /**
     * Returns stage.
     *
     * @return stage
     */
    public String getStage() {
        return stage;
    }

    @Override
    public String toString() {
        return                 "FundOrderStage [desc=" + desc +
                ", key=" + key +
                ", link=" + link +
                ", linkText=" + linkText +
                ", progress=" + progress +
                ", stage=" + stage + "]";
    }
}
