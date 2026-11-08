package com.longbridge.grid;

import java.math.BigDecimal;

import java.time.OffsetDateTime;

/**
 * A grid trading order
 */
public class GridOrder {
    private String orderId;
    private String symbol;
    private String stockName;
    private String market;
    private String status;
    private String gridStatus;
    private BigDecimal submittedBasePrice;
    private BigDecimal currentBasePrice;
    private BigDecimal preTriggerBasePrice;
    private BigDecimal postTriggerBasePrice;
    private BigDecimal upperLimitPrice;
    private BigDecimal lowerLimitPrice;
    private TriggerPriceType triggerPriceType;
    private BigDecimal triggerSpreadUp;
    private BigDecimal triggerSpreadDown;
    private BigDecimal triggerPercentUp;
    private BigDecimal triggerPercentDown;
    private BigDecimal pullbackPercent;
    private BigDecimal pullbackSpread;
    private BigDecimal reboundPercent;
    private BigDecimal reboundSpread;
    private String triggerSellOrderType;
    private String triggerBuyOrderType;
    private int triggerSellDepth;
    private int triggerBuyDepth;
    private BigDecimal triggerQuantity;
    private BigDecimal triggerSellQuantity;
    private BigDecimal triggerBuyQuantity;
    private BigDecimal upperLimitQuantity;
    private BigDecimal lowerLimitQuantity;
    private GridLimitEvent upperLimitEvent;
    private GridLimitEvent lowerLimitEvent;
    private boolean multipleTrigger;
    private int triggerTimes;
    private BigDecimal totalBuyQuantity;
    private BigDecimal totalSellQuantity;
    private BigDecimal totalProfitBalance;
    private String settlementCurrency;
    private GridTimeInForce timeInForce;
    private String gtd;
    private OffsetDateTime createdAt;
    private int rth;
    private boolean supportShortsell;
    private String gridOrderTypeUp;
    private String gridOrderTypeDown;

    /**
     * Returns orderId.
     *
     * @return orderId
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns symbol.
     *
     * @return symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns stockName.
     *
     * @return stockName
     */
    public String getStockName() {
        return stockName;
    }

    /**
     * Returns market.
     *
     * @return market
     */
    public String getMarket() {
        return market;
    }

    /**
     * Returns status.
     *
     * @return status
     */
    public String getStatus() {
        return status;
    }

    /**
     * Returns gridStatus.
     *
     * @return gridStatus
     */
    public String getGridStatus() {
        return gridStatus;
    }

    /**
     * Returns submittedBasePrice.
     *
     * @return submittedBasePrice
     */
    public BigDecimal getSubmittedBasePrice() {
        return submittedBasePrice;
    }

    /**
     * Returns currentBasePrice.
     *
     * @return currentBasePrice
     */
    public BigDecimal getCurrentBasePrice() {
        return currentBasePrice;
    }

    /**
     * Returns preTriggerBasePrice.
     *
     * @return preTriggerBasePrice
     */
    public BigDecimal getPreTriggerBasePrice() {
        return preTriggerBasePrice;
    }

    /**
     * Returns postTriggerBasePrice.
     *
     * @return postTriggerBasePrice
     */
    public BigDecimal getPostTriggerBasePrice() {
        return postTriggerBasePrice;
    }

    /**
     * Returns upperLimitPrice.
     *
     * @return upperLimitPrice
     */
    public BigDecimal getUpperLimitPrice() {
        return upperLimitPrice;
    }

    /**
     * Returns lowerLimitPrice.
     *
     * @return lowerLimitPrice
     */
    public BigDecimal getLowerLimitPrice() {
        return lowerLimitPrice;
    }

    /**
     * Returns triggerPriceType.
     *
     * @return triggerPriceType
     */
    public TriggerPriceType getTriggerPriceType() {
        return triggerPriceType;
    }

    /**
     * Returns triggerSpreadUp.
     *
     * @return triggerSpreadUp
     */
    public BigDecimal getTriggerSpreadUp() {
        return triggerSpreadUp;
    }

    /**
     * Returns triggerSpreadDown.
     *
     * @return triggerSpreadDown
     */
    public BigDecimal getTriggerSpreadDown() {
        return triggerSpreadDown;
    }

    /**
     * Returns triggerPercentUp.
     *
     * @return triggerPercentUp
     */
    public BigDecimal getTriggerPercentUp() {
        return triggerPercentUp;
    }

    /**
     * Returns triggerPercentDown.
     *
     * @return triggerPercentDown
     */
    public BigDecimal getTriggerPercentDown() {
        return triggerPercentDown;
    }

    /**
     * Returns pullbackPercent.
     *
     * @return pullbackPercent
     */
    public BigDecimal getPullbackPercent() {
        return pullbackPercent;
    }

    /**
     * Returns pullbackSpread.
     *
     * @return pullbackSpread
     */
    public BigDecimal getPullbackSpread() {
        return pullbackSpread;
    }

    /**
     * Returns reboundPercent.
     *
     * @return reboundPercent
     */
    public BigDecimal getReboundPercent() {
        return reboundPercent;
    }

    /**
     * Returns reboundSpread.
     *
     * @return reboundSpread
     */
    public BigDecimal getReboundSpread() {
        return reboundSpread;
    }

    /**
     * Returns triggerSellOrderType.
     *
     * @return triggerSellOrderType
     */
    public String getTriggerSellOrderType() {
        return triggerSellOrderType;
    }

    /**
     * Returns triggerBuyOrderType.
     *
     * @return triggerBuyOrderType
     */
    public String getTriggerBuyOrderType() {
        return triggerBuyOrderType;
    }

    /**
     * Returns triggerSellDepth.
     *
     * @return triggerSellDepth
     */
    public int getTriggerSellDepth() {
        return triggerSellDepth;
    }

    /**
     * Returns triggerBuyDepth.
     *
     * @return triggerBuyDepth
     */
    public int getTriggerBuyDepth() {
        return triggerBuyDepth;
    }

    /**
     * Returns triggerQuantity.
     *
     * @return triggerQuantity
     */
    public BigDecimal getTriggerQuantity() {
        return triggerQuantity;
    }

    /**
     * Returns triggerSellQuantity.
     *
     * @return triggerSellQuantity
     */
    public BigDecimal getTriggerSellQuantity() {
        return triggerSellQuantity;
    }

    /**
     * Returns triggerBuyQuantity.
     *
     * @return triggerBuyQuantity
     */
    public BigDecimal getTriggerBuyQuantity() {
        return triggerBuyQuantity;
    }

    /**
     * Returns upperLimitQuantity.
     *
     * @return upperLimitQuantity
     */
    public BigDecimal getUpperLimitQuantity() {
        return upperLimitQuantity;
    }

    /**
     * Returns lowerLimitQuantity.
     *
     * @return lowerLimitQuantity
     */
    public BigDecimal getLowerLimitQuantity() {
        return lowerLimitQuantity;
    }

    /**
     * Returns upperLimitEvent.
     *
     * @return upperLimitEvent
     */
    public GridLimitEvent getUpperLimitEvent() {
        return upperLimitEvent;
    }

    /**
     * Returns lowerLimitEvent.
     *
     * @return lowerLimitEvent
     */
    public GridLimitEvent getLowerLimitEvent() {
        return lowerLimitEvent;
    }

    /**
     * Returns multipleTrigger.
     *
     * @return multipleTrigger
     */
    public boolean getMultipleTrigger() {
        return multipleTrigger;
    }

    /**
     * Returns triggerTimes.
     *
     * @return triggerTimes
     */
    public int getTriggerTimes() {
        return triggerTimes;
    }

    /**
     * Returns totalBuyQuantity.
     *
     * @return totalBuyQuantity
     */
    public BigDecimal getTotalBuyQuantity() {
        return totalBuyQuantity;
    }

    /**
     * Returns totalSellQuantity.
     *
     * @return totalSellQuantity
     */
    public BigDecimal getTotalSellQuantity() {
        return totalSellQuantity;
    }

    /**
     * Returns totalProfitBalance.
     *
     * @return totalProfitBalance
     */
    public BigDecimal getTotalProfitBalance() {
        return totalProfitBalance;
    }

    /**
     * Returns settlementCurrency.
     *
     * @return settlementCurrency
     */
    public String getSettlementCurrency() {
        return settlementCurrency;
    }

    /**
     * Returns timeInForce.
     *
     * @return timeInForce
     */
    public GridTimeInForce getTimeInForce() {
        return timeInForce;
    }

    /**
     * Returns gtd.
     *
     * @return gtd
     */
    public String getGtd() {
        return gtd;
    }

    /**
     * Returns createdAt.
     *
     * @return createdAt
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns rth.
     *
     * @return rth
     */
    public int getRth() {
        return rth;
    }

    /**
     * Returns supportShortsell.
     *
     * @return supportShortsell
     */
    public boolean getSupportShortsell() {
        return supportShortsell;
    }

    /**
     * Returns gridOrderTypeUp.
     *
     * @return gridOrderTypeUp
     */
    public String getGridOrderTypeUp() {
        return gridOrderTypeUp;
    }

    /**
     * Returns gridOrderTypeDown.
     *
     * @return gridOrderTypeDown
     */
    public String getGridOrderTypeDown() {
        return gridOrderTypeDown;
    }

    @Override
    public String toString() {
        return "GridOrder [orderId=" + orderId +
                ", symbol=" + symbol +
                ", stockName=" + stockName +
                ", market=" + market +
                ", status=" + status +
                ", gridStatus=" + gridStatus +
                ", submittedBasePrice=" + submittedBasePrice +
                ", currentBasePrice=" + currentBasePrice +
                ", preTriggerBasePrice=" + preTriggerBasePrice +
                ", postTriggerBasePrice=" + postTriggerBasePrice +
                ", upperLimitPrice=" + upperLimitPrice +
                ", lowerLimitPrice=" + lowerLimitPrice +
                ", triggerPriceType=" + triggerPriceType +
                ", triggerSpreadUp=" + triggerSpreadUp +
                ", triggerSpreadDown=" + triggerSpreadDown +
                ", triggerPercentUp=" + triggerPercentUp +
                ", triggerPercentDown=" + triggerPercentDown +
                ", pullbackPercent=" + pullbackPercent +
                ", pullbackSpread=" + pullbackSpread +
                ", reboundPercent=" + reboundPercent +
                ", reboundSpread=" + reboundSpread +
                ", triggerSellOrderType=" + triggerSellOrderType +
                ", triggerBuyOrderType=" + triggerBuyOrderType +
                ", triggerSellDepth=" + triggerSellDepth +
                ", triggerBuyDepth=" + triggerBuyDepth +
                ", triggerQuantity=" + triggerQuantity +
                ", triggerSellQuantity=" + triggerSellQuantity +
                ", triggerBuyQuantity=" + triggerBuyQuantity +
                ", upperLimitQuantity=" + upperLimitQuantity +
                ", lowerLimitQuantity=" + lowerLimitQuantity +
                ", upperLimitEvent=" + upperLimitEvent +
                ", lowerLimitEvent=" + lowerLimitEvent +
                ", multipleTrigger=" + multipleTrigger +
                ", triggerTimes=" + triggerTimes +
                ", totalBuyQuantity=" + totalBuyQuantity +
                ", totalSellQuantity=" + totalSellQuantity +
                ", totalProfitBalance=" + totalProfitBalance +
                ", settlementCurrency=" + settlementCurrency +
                ", timeInForce=" + timeInForce +
                ", gtd=" + gtd +
                ", createdAt=" + createdAt +
                ", rth=" + rth +
                ", supportShortsell=" + supportShortsell +
                ", gridOrderTypeUp=" + gridOrderTypeUp +
                ", gridOrderTypeDown=" + gridOrderTypeDown + "]";
    }
}
