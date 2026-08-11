package com.longbridge.grid;

import java.util.Arrays;

/**
 * Order info fields used by the grid order window
 */
public class GridOrderInfo {
    private String name;
    private String lastDone;
    private String lotSize;
    private String buyLotSize;
    private String sellLotSize;
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
    public String getLastDone() {
        return lastDone;
    }

    /**
     * Returns lotSize.
     *
     * @return lotSize
     */
    public String getLotSize() {
        return lotSize;
    }

    /**
     * Returns buyLotSize.
     *
     * @return buyLotSize
     */
    public String getBuyLotSize() {
        return buyLotSize;
    }

    /**
     * Returns sellLotSize.
     *
     * @return sellLotSize
     */
    public String getSellLotSize() {
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
