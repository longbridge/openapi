package com.longbridge.dca;

/** Options for {@link DcaContext#list}. */
public class DcaListOptions {
    /** Filter by plan status: 0=Active, 1=Suspended, 2=Finished (optional). */
    public Integer status;
    /** Filter by security symbol (optional). */
    public String symbol;
}
