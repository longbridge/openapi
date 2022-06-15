package com.longbridge.quote;

import java.time.LocalDate;

public class MarketTradingDays {
    private LocalDate[] tradingDays;
    private LocalDate[] halfTradingDays;

    public LocalDate[] getTradingDays() {
        return tradingDays;
    }

    public LocalDate[] getHalfTradingDays() {
        return halfTradingDays;
    }
}
