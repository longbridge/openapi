package com.longbridge.asset;

import java.lang.reflect.Type;
import java.util.Arrays;
import java.util.Collections;
import java.util.Map;
import java.util.TreeMap;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;

/**
 * Terms of a stock fragment coupon ({@code stock_fragment})
 */
public class CouponStockFragment {
    private static final Gson GSON = new Gson();
    private static final Type STRING_MAP = new TypeToken<Map<String, String>>() {
    }.getType();

    private String counterId;
    private String counterName;
    private String market;
    private String[] markets;
    private CouponTargetType underlyingType;
    // JSON object text, parsed on access
    private String amounts;
    private transient Map<String, String> amountsMap;

    /**
     * Returns the counter ID.
     *
     * @return the counter ID
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns the counter name.
     *
     * @return the counter name
     */
    public String getCounterName() {
        return counterName;
    }

    /**
     * Returns the market.
     *
     * @return the market
     */
    public String getMarket() {
        return market;
    }

    /**
     * Returns the markets.
     *
     * @return the markets
     */
    public String[] getMarkets() {
        return markets;
    }

    /**
     * Returns the instrument type the coupon applies to.
     *
     * @return the instrument type the coupon applies to
     */
    public CouponTargetType getUnderlyingType() {
        return underlyingType;
    }

    /**
     * Returns the amounts, as returned by the server.
     *
     * @return the amounts, in key order (read-only); empty when the server returns none
     */
    public synchronized Map<String, String> getAmounts() {
        if (amountsMap == null) {
            Map<String, String> parsed = amounts == null || amounts.isEmpty() ? null : GSON.fromJson(amounts, STRING_MAP);
            amountsMap = parsed == null ? Collections.emptyMap() : Collections.unmodifiableMap(new TreeMap<>(parsed));
        }
        return amountsMap;
    }

    @Override
    public String toString() {
        return "CouponStockFragment [" + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "market=" + market + ", " + "markets=" + Arrays.toString(markets) + ", " + "underlyingType=" + underlyingType + ", " + "amounts=" + getAmounts() + "]";
    }
}
