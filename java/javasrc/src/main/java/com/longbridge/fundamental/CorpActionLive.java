package com.longbridge.fundamental;

/** Live stream associated with a corporate action. */
public class CorpActionLive {
    /** Live stream ID. */
    public String id;
    /** Status code: 1=preview, 2=live, 3=ended, 4=replay, 5=processing. */
    public String status;
    /** Start time. */
    public String startedAt;
    /** Stream title. */
    public String name;
    /** Icon URL. */
    public String icon;
}