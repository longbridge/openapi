package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

import com.longbridge.Market;

public class WatchListSecurity {
    private String symbol;
    private Market market;
    private String name;
    private BigDecimal price;
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

    public BigDecimal getPrice() {
        return price;
    }

    public OffsetDateTime getWatchedAt() {
        return watchedAt;
    }

    @Override
    public String toString() {
        return "WatchListSecurity [market=" + market + ", name=" + name + ", price=" + price + ", symbol=" + symbol
                + ", watchedAt=" + watchedAt + "]";
    }

}
