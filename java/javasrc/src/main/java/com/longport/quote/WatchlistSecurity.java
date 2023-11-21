package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

import com.longport.Market;

public class WatchlistSecurity {
    private String symbol;
    private Market market;
    private String name;
    private BigDecimal watchedPrice;
    private OffsetDateTime watchedAt;

    public String getSymbol() {
        return symbol;
    }

    public Market getMarket() {
        return market;
    }

    public String getName() {
        return name;
    }

    public BigDecimal getWatchedPrice() {
        return watchedPrice;
    }

    public OffsetDateTime getWatchedAt() {
        return watchedAt;
    }

    @Override
    public String toString() {
        return "WatchlistSecurity [market=" + market + ", name=" + name + ", watchedPrice=" + watchedPrice + ", symbol="
                + symbol
                + ", watchedAt=" + watchedAt + "]";
    }

}
