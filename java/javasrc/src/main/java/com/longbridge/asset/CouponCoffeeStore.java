package com.longbridge.asset;

/**
 * A store of a coffee coupon
 */
public class CouponCoffeeStore {
    private long storeId;
    private String storeName;

    /**
     * Returns the store ID.
     *
     * @return the store ID
     */
    public long getStoreId() {
        return storeId;
    }

    /**
     * Returns the store name.
     *
     * @return the store name
     */
    public String getStoreName() {
        return storeName;
    }

    @Override
    public String toString() {
        return "CouponCoffeeStore [" + "storeId=" + storeId + ", " + "storeName=" + storeName + "]";
    }
}
