package com.longbridge.fundamental;

/** One corporate action event. */
public class CorpActionItem {
    /** Internal event ID. */
    public String id;
    /** Date in {@code YYYYMMDD} format, e.g. {@code "20260601"}. */
    public String date;
    /** Short display date, e.g. {@code "06.01"}. */
    public String dateStr;
    /** Date type label, e.g. {@code "派息日"}, {@code "除权日"}. */
    public String dateType;
    /** Time zone description, e.g. {@code "北京时间"}. */
    public String dateZone;
    /** Event category, e.g. {@code "分配方案"}. */
    public String actType;
    /** Human-readable event description. */
    public String actDesc;
    /** Machine-readable action code, e.g. {@code "DividendExDate"}. */
    public String action;
    /** Whether this is a recent event. */
    public boolean recent;
    /** Whether publication was delayed. */
    public boolean isDelay;
    /** Delay announcement content (if {@code isDelay} is {@code true}). */
    public String delayContent;
    /** Associated live stream (if any). */
    public CorpActionLive live;
    /** Associated security info (rarely populated; raw JSON string). */
    public String security;
}