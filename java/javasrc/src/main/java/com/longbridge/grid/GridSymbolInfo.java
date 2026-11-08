package com.longbridge.grid;

import java.math.BigDecimal;

import java.util.Arrays;

/**
 * Security (symbol) info used to build a grid order
 */
public class GridSymbolInfo {
    private String name;
    private BigDecimal lastDone;
    private BigDecimal lotSize;
    private BigDecimal buyLotSize;
    private BigDecimal sellLotSize;
    private GridBidSize[] bidSizes;
    private GridChannelInfo channelInfo;

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns lastDone.
     *
     * @return lastDone
     */
    public BigDecimal getLastDone() {
        return lastDone;
    }

    /**
     * Returns lotSize.
     *
     * @return lotSize
     */
    public BigDecimal getLotSize() {
        return lotSize;
    }

    /**
     * Returns buyLotSize.
     *
     * @return buyLotSize
     */
    public BigDecimal getBuyLotSize() {
        return buyLotSize;
    }

    /**
     * Returns sellLotSize.
     *
     * @return sellLotSize
     */
    public BigDecimal getSellLotSize() {
        return sellLotSize;
    }

    /**
     * Returns bidSizes.
     *
     * @return bidSizes
     */
    public GridBidSize[] getBidSizes() {
        return bidSizes;
    }

    /**
     * Returns channelInfo.
     *
     * @return channelInfo
     */
    public GridChannelInfo getChannelInfo() {
        return channelInfo;
    }

    @Override
    public String toString() {
        return "GridSymbolInfo [name=" + name +
                ", lastDone=" + lastDone +
                ", lotSize=" + lotSize +
                ", buyLotSize=" + buyLotSize +
                ", sellLotSize=" + sellLotSize +
                ", bidSizes=" + Arrays.toString(bidSizes) +
                ", channelInfo=" + channelInfo + "]";
    }
}
