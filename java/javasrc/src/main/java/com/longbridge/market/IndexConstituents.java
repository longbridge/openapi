package com.longbridge.market;

/** Constituent stocks of a market index with daily movement summary. */
public class IndexConstituents {
    /** Number of constituent stocks that fell today. */
    public int fallNum;
    /** Number of constituent stocks unchanged today. */
    public int flatNum;
    /** Number of constituent stocks that rose today. */
    public int riseNum;
    /** Constituent stock details. */
    public ConstituentStock[] stocks;
}