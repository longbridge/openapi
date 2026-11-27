package com.longbridge.asset;

/**
 * Options for the coupon list (lists coupons that have not expired unless
 * {@link #setExpired(boolean)} is set)
 */
@SuppressWarnings("unused")
public class GetCouponsOptions {
    private Integer page;
    private Integer pageSize;
    private boolean expired;
    private String cardType;

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetCouponsOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (max 50).
     *
     * @param pageSize page size
     * @return this instance for chaining
     */
    public GetCouponsOptions setPageSize(int pageSize) {
        this.pageSize = pageSize;
        return this;
    }

    /**
     * Lists expired coupons instead of the ones that have not expired.
     *
     * @param expired whether to list expired coupons
     * @return this instance for chaining
     */
    public GetCouponsOptions setExpired(boolean expired) {
        this.expired = expired;
        return this;
    }

    /**
     * Filters by coupon type (e.g. {@code commission},
     * {@code platform_fee_deduct}, {@code stock}, {@code quote}, {@code cash};
     * default: all types).
     *
     * @param cardType coupon type
     * @return this instance for chaining
     */
    public GetCouponsOptions setCardType(String cardType) {
        this.cardType = cardType;
        return this;
    }
}
