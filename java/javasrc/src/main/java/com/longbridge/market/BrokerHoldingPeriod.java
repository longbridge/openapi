package com.longbridge.market;

/** Lookback period for broker holding net change. */
public enum BrokerHoldingPeriod {
    /** 1 recent trading day */
    Rct1,
    /** 5 recent trading days */
    Rct5,
    /** 20 recent trading days */
    Rct20,
    /** 60 recent trading days */
    Rct60,
}
