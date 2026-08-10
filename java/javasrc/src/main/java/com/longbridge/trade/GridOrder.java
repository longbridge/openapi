package com.longbridge.trade;

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
    private String submittedBasePrice;
    private String currentBasePrice;
    private String preTriggerBasePrice;
    private String postTriggerBasePrice;
    private String upperLimitPrice;
    private String lowerLimitPrice;
    private int triggerPriceType;
    private String triggerSpreadUp;
    private String triggerSpreadDown;
    private String triggerPercentUp;
    private String triggerPercentDown;
    private String pullbackPercent;
    private String pullbackSpread;
    private String reboundPercent;
    private String reboundSpread;
    private String triggerSellOrderType;
    private String triggerBuyOrderType;
    private int triggerSellDepth;
    private int triggerBuyDepth;
    private String triggerQuantity;
    private String triggerSellQuantity;
    private String triggerBuyQuantity;
    private String upperLimitQuantity;
    private String lowerLimitQuantity;
    private int upperLimitEvent;
    private int lowerLimitEvent;
    private boolean multipleTrigger;
    private int triggerTimes;
    private String totalBuyQuantity;
    private String totalSellQuantity;
    private String totalProfitBalance;
    private String settlementCurrency;
    private int timeInForce;
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
    public String getSubmittedBasePrice() {
        return submittedBasePrice;
    }

    /**
     * Returns currentBasePrice.
     *
     * @return currentBasePrice
     */
    public String getCurrentBasePrice() {
        return currentBasePrice;
    }

    /**
     * Returns preTriggerBasePrice.
     *
     * @return preTriggerBasePrice
     */
    public String getPreTriggerBasePrice() {
        return preTriggerBasePrice;
    }

    /**
     * Returns postTriggerBasePrice.
     *
     * @return postTriggerBasePrice
     */
    public String getPostTriggerBasePrice() {
        return postTriggerBasePrice;
    }

    /**
     * Returns upperLimitPrice.
     *
     * @return upperLimitPrice
     */
    public String getUpperLimitPrice() {
        return upperLimitPrice;
    }

    /**
     * Returns lowerLimitPrice.
     *
     * @return lowerLimitPrice
     */
    public String getLowerLimitPrice() {
        return lowerLimitPrice;
    }

    /**
     * Returns triggerPriceType.
     *
     * @return triggerPriceType
     */
    public int getTriggerPriceType() {
        return triggerPriceType;
    }

    /**
     * Returns triggerSpreadUp.
     *
     * @return triggerSpreadUp
     */
    public String getTriggerSpreadUp() {
        return triggerSpreadUp;
    }

    /**
     * Returns triggerSpreadDown.
     *
     * @return triggerSpreadDown
     */
    public String getTriggerSpreadDown() {
        return triggerSpreadDown;
    }

    /**
     * Returns triggerPercentUp.
     *
     * @return triggerPercentUp
     */
    public String getTriggerPercentUp() {
        return triggerPercentUp;
    }

    /**
     * Returns triggerPercentDown.
     *
     * @return triggerPercentDown
     */
    public String getTriggerPercentDown() {
        return triggerPercentDown;
    }

    /**
     * Returns pullbackPercent.
     *
     * @return pullbackPercent
     */
    public String getPullbackPercent() {
        return pullbackPercent;
    }

    /**
     * Returns pullbackSpread.
     *
     * @return pullbackSpread
     */
    public String getPullbackSpread() {
        return pullbackSpread;
    }

    /**
     * Returns reboundPercent.
     *
     * @return reboundPercent
     */
    public String getReboundPercent() {
        return reboundPercent;
    }

    /**
     * Returns reboundSpread.
     *
     * @return reboundSpread
     */
    public String getReboundSpread() {
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
    public String getTriggerQuantity() {
        return triggerQuantity;
    }

    /**
     * Returns triggerSellQuantity.
     *
     * @return triggerSellQuantity
     */
    public String getTriggerSellQuantity() {
        return triggerSellQuantity;
    }

    /**
     * Returns triggerBuyQuantity.
     *
     * @return triggerBuyQuantity
     */
    public String getTriggerBuyQuantity() {
        return triggerBuyQuantity;
    }

    /**
     * Returns upperLimitQuantity.
     *
     * @return upperLimitQuantity
     */
    public String getUpperLimitQuantity() {
        return upperLimitQuantity;
    }

    /**
     * Returns lowerLimitQuantity.
     *
     * @return lowerLimitQuantity
     */
    public String getLowerLimitQuantity() {
        return lowerLimitQuantity;
    }

    /**
     * Returns upperLimitEvent.
     *
     * @return upperLimitEvent
     */
    public int getUpperLimitEvent() {
        return upperLimitEvent;
    }

    /**
     * Returns lowerLimitEvent.
     *
     * @return lowerLimitEvent
     */
    public int getLowerLimitEvent() {
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
    public String getTotalBuyQuantity() {
        return totalBuyQuantity;
    }

    /**
     * Returns totalSellQuantity.
     *
     * @return totalSellQuantity
     */
    public String getTotalSellQuantity() {
        return totalSellQuantity;
    }

    /**
     * Returns totalProfitBalance.
     *
     * @return totalProfitBalance
     */
    public String getTotalProfitBalance() {
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
    public int getTimeInForce() {
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
