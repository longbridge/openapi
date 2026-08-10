package com.longbridge.trade;

import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * Detail of a grid trading order
 */
public class GridOrderDetail {
    private String orderId;
    private String symbol;
    private String stockName;
    private String status;
    private String gridStatus;
    private String suspendReason;
    private String sleepingReason;
    private String submittedBasePrice;
    private String currentBasePrice;
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
    private boolean multipleTrigger;
    private int timeInForce;
    private String triggerQuantity;
    private String triggerSellQuantity;
    private String triggerBuyQuantity;
    private String upperLimitQuantity;
    private String lowerLimitQuantity;
    private int upperLimitEvent;
    private int lowerLimitEvent;
    private int triggerSellDepth;
    private int triggerBuyDepth;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private String settlementCurrency;
    private OffsetDateTime expireTime;
    private String gtd;
    private GridOrderSubOrder[] gridSubOrders;
    private boolean subHasMore;
    private GridOrderHistory[] gridOrderHistory;
    private boolean historyHasMore;
    private boolean supportShortsell;
    private int rth;
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
     * Returns suspendReason.
     *
     * @return suspendReason
     */
    public String getSuspendReason() {
        return suspendReason;
    }

    /**
     * Returns sleepingReason.
     *
     * @return sleepingReason
     */
    public String getSleepingReason() {
        return sleepingReason;
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
     * Returns multipleTrigger.
     *
     * @return multipleTrigger
     */
    public boolean getMultipleTrigger() {
        return multipleTrigger;
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
     * Returns createdAt.
     *
     * @return createdAt
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns updatedAt.
     *
     * @return updatedAt
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
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
     * Returns expireTime.
     *
     * @return expireTime
     */
    public OffsetDateTime getExpireTime() {
        return expireTime;
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
     * Returns gridSubOrders.
     *
     * @return gridSubOrders
     */
    public GridOrderSubOrder[] getGridSubOrders() {
        return gridSubOrders;
    }

    /**
     * Returns subHasMore.
     *
     * @return subHasMore
     */
    public boolean getSubHasMore() {
        return subHasMore;
    }

    /**
     * Returns gridOrderHistory.
     *
     * @return gridOrderHistory
     */
    public GridOrderHistory[] getGridOrderHistory() {
        return gridOrderHistory;
    }

    /**
     * Returns historyHasMore.
     *
     * @return historyHasMore
     */
    public boolean getHistoryHasMore() {
        return historyHasMore;
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
     * Returns rth.
     *
     * @return rth
     */
    public int getRth() {
        return rth;
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
        return "GridOrderDetail [orderId=" + orderId +
                ", symbol=" + symbol +
                ", stockName=" + stockName +
                ", status=" + status +
                ", gridStatus=" + gridStatus +
                ", suspendReason=" + suspendReason +
                ", sleepingReason=" + sleepingReason +
                ", submittedBasePrice=" + submittedBasePrice +
                ", currentBasePrice=" + currentBasePrice +
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
                ", multipleTrigger=" + multipleTrigger +
                ", timeInForce=" + timeInForce +
                ", triggerQuantity=" + triggerQuantity +
                ", triggerSellQuantity=" + triggerSellQuantity +
                ", triggerBuyQuantity=" + triggerBuyQuantity +
                ", upperLimitQuantity=" + upperLimitQuantity +
                ", lowerLimitQuantity=" + lowerLimitQuantity +
                ", upperLimitEvent=" + upperLimitEvent +
                ", lowerLimitEvent=" + lowerLimitEvent +
                ", triggerSellDepth=" + triggerSellDepth +
                ", triggerBuyDepth=" + triggerBuyDepth +
                ", createdAt=" + createdAt +
                ", updatedAt=" + updatedAt +
                ", settlementCurrency=" + settlementCurrency +
                ", expireTime=" + expireTime +
                ", gtd=" + gtd +
                ", gridSubOrders=" + Arrays.toString(gridSubOrders) +
                ", subHasMore=" + subHasMore +
                ", gridOrderHistory=" + Arrays.toString(gridOrderHistory) +
                ", historyHasMore=" + historyHasMore +
                ", supportShortsell=" + supportShortsell +
                ", rth=" + rth +
                ", gridOrderTypeUp=" + gridOrderTypeUp +
                ", gridOrderTypeDown=" + gridOrderTypeDown + "]";
    }
}
