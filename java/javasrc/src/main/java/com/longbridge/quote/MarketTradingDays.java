package com.longbridge.quote;

import java.time.LocalDate;
import java.util.Arrays;

public class MarketTradingDays {
    private LocalDate[] tradingDays;
    private LocalDate[] halfTradingDays;

    public LocalDate[] getTradingDays() {
        return tradingDays;
    }

    public LocalDate[] getHalfTradingDays() {
        return halfTradingDays;
    }

    @Override
    public String toString() {
        return "MarketTradingDays [halfTradingDays=" + Arrays.toString(halfTradingDays) + ", tradingDays="
                + Arrays.toString(tradingDays) + "]";
    }
}
