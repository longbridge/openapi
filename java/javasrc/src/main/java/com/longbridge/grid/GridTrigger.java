package com.longbridge.grid;

import java.math.BigDecimal;

/**
 * How a grid's up/down trigger thresholds are expressed. Percent and spread are
 * mutually exclusive; use the {@link #percent} / {@link #spread} factories to
 * make the choice explicit instead of setting four independent fields.
 */
public final class GridTrigger {
    private final TriggerPriceType type;
    private final BigDecimal up;
    private final BigDecimal down;

    private GridTrigger(TriggerPriceType type, BigDecimal up, BigDecimal down) {
        this.type = type;
        this.up = up;
        this.down = down;
    }

    /**
     * Trigger by percent.
     *
     * @param up   upward trigger percent
     * @param down downward trigger percent
     * @return a percent trigger
     */
    public static GridTrigger percent(BigDecimal up, BigDecimal down) {
        return new GridTrigger(TriggerPriceType.Percent, up, down);
    }

    /**
     * Trigger by absolute price spread.
     *
     * @param up   upward trigger spread
     * @param down downward trigger spread
     * @return a spread trigger
     */
    public static GridTrigger spread(BigDecimal up, BigDecimal down) {
        return new GridTrigger(TriggerPriceType.Spread, up, down);
    }

    /**
     * Returns the trigger price type (percent or spread).
     *
     * @return the trigger price type
     */
    public TriggerPriceType getType() {
        return type;
    }

    /**
     * Returns the upward threshold.
     *
     * @return upward threshold
     */
    public BigDecimal getUp() {
        return up;
    }

    /**
     * Returns the downward threshold.
     *
     * @return downward threshold
     */
    public BigDecimal getDown() {
        return down;
    }

    @Override
    public String toString() {
        return "GridTrigger [type=" + type + ", up=" + up + ", down=" + down + "]";
    }
}
