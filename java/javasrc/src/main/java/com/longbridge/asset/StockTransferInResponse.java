package com.longbridge.asset;

import java.util.Arrays;

/**
 * Response for the stock transfer-in list
 */
public class StockTransferInResponse {
    private StockTransferIn[] records;
    private long total;

    /**
     * Returns the records.
     *
     * @return the records
     */
    public StockTransferIn[] getRecords() {
        return records;
    }

    /**
     * Returns the total number of records.
     *
     * @return the total number of records
     */
    public long getTotal() {
        return total;
    }

    @Override
    public String toString() {
        return "StockTransferInResponse [" + "records=" + Arrays.toString(records) + ", " + "total=" + total + "]";
    }
}
