package com.longbridge.fundamental;

/**
 * A node in the recursive industry peer chain.
 *
 * <p>{@code nextJson} contains the child nodes serialised as a JSON string.
 */
public class IndustryPeerNode {
    /** Node name */
    public String name;
    /** Counter ID */
    public String counterId;
    /** Number of stocks in this node */
    public int stockNum;
    /** Change percentage */
    public String chg;
    /** Year-to-date change */
    public String ytdChg;
    /** Child nodes as a JSON string */
    public String nextJson;
}
