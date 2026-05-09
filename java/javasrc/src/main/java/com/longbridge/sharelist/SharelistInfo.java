package com.longbridge.sharelist;

import java.time.OffsetDateTime;

/** Sharelist information. */
public class SharelistInfo {
    /** Sharelist ID. */
    public long id;
    /** Name. */
    public String name;
    /** Description. */
    public String description;
    /** Cover image URL. */
    public String cover;
    /** Number of subscribers. */
    public long subscribersCount;
    /** Creation time. */
    public OffsetDateTime createdAt;
    /** Last stock edit time. */
    public OffsetDateTime editedAt;
    /** YTD change percentage. */
    public java.math.BigDecimal thisYearChg;
    /** Creator info. */
    public String creator;
    /** Constituent stocks. */
    public SharelistStock[] stocks;
    /** Whether the current user is subscribed. */
    public boolean subscribed;
    /** Day change percentage. */
    public java.math.BigDecimal chg;
    /** Sharelist type: 0=regular, 3=official, 4=industry. */
    public int sharelistType;
    /** Industry code (for industry sharelists). */
    public String industryCode;
}
