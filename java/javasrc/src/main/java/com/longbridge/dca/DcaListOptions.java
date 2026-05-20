package com.longbridge.dca;

/** Options for {@link DcaContext#list}. */
public class DcaListOptions {
    /** Filter by plan status (optional). */
    public DCAStatus status;
    /** Filter by security symbol (optional). */
    public String symbol;
}
