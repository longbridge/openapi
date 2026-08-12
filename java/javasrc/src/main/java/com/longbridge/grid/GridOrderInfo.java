package com.longbridge.grid;

import java.math.BigDecimal;

import java.util.Arrays;

/**
 * Order info fields used by the grid order window
 */
public class GridOrderInfo {
    private String name;
    private BigDecimal lastDone;
    private BigDecimal lotSize;
    private BigDecimal buyLotSize;
    private BigDecimal sellLotSize;
    private GridBidSize[] bidSizes;
    private GridChannelInfo channelInfos;

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
     * Returns channelInfos.
     *
     * @return channelInfos
     */
    public GridChannelInfo getChannelInfos() {
        return channelInfos;
    }

    @Override
    public String toString() {
        return "GridOrderInfo [name=" + name +
                ", lastDone=" + lastDone +
                ", lotSize=" + lotSize +
                ", buyLotSize=" + buyLotSize +
                ", sellLotSize=" + sellLotSize +
                ", bidSizes=" + Arrays.toString(bidSizes) +
                ", channelInfos=" + channelInfos + "]";
    }
}
