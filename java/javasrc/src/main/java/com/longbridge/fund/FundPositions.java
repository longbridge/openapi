package com.longbridge.fund;

/**
 * FundPositions
 */
public class FundPositions {
    private String accountChannel;
    private FundPosition[] list;
    private String pendingBuyOrders;
    private long recentTradingDay;
    private String soldPendingCreditOrders;

    /**
     * Returns accountChannel.
     *
     * @return accountChannel
     */
    public String getAccountChannel() {
        return accountChannel;
    }

    /**
     * Returns list.
     *
     * @return list
     */
    public FundPosition[] getList() {
        return list;
    }

    /**
     * Returns pendingBuyOrders.
     *
     * @return pendingBuyOrders
     */
    public String getPendingBuyOrders() {
        return pendingBuyOrders;
    }

    /**
     * Returns recentTradingDay.
     *
     * @return recentTradingDay
     */
    public long getRecentTradingDay() {
        return recentTradingDay;
    }

    /**
     * Returns soldPendingCreditOrders.
     *
     * @return soldPendingCreditOrders
     */
    public String getSoldPendingCreditOrders() {
        return soldPendingCreditOrders;
    }

    @Override
    public String toString() {
        return                 "FundPositions [accountChannel=" + accountChannel +
                ", list=" + java.util.Arrays.toString(list) +
                ", pendingBuyOrders=" + pendingBuyOrders +
                ", recentTradingDay=" + recentTradingDay +
                ", soldPendingCreditOrders=" + soldPendingCreditOrders + "]";
    }
}
