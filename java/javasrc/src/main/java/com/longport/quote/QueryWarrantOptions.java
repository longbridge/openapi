package com.longport.quote;

@SuppressWarnings("unused")
public class QueryWarrantOptions {
    private String symbol;
    private WarrantSortBy sortBy;
    private SortOrderType sortType;
    private WarrantType[] warrantType;
    private int[] issuer;
    private FilterWarrantExpiryDate[] expiryDate;
    private FilterWarrantInOutBoundsType[] priceType;
    private WarrantStatus[] status;

    public QueryWarrantOptions(String symbol, WarrantSortBy sortBy, SortOrderType sortType) {
        this.symbol = symbol;
        this.sortBy = sortBy;
        this.sortType = sortType;
    }

    public QueryWarrantOptions setWarrantType(WarrantType[] warrantType) {
        this.warrantType = warrantType;
        return this;
    }

    public QueryWarrantOptions setIssuer(int[] issuer) {
        this.issuer = issuer;
        return this;
    }

    public QueryWarrantOptions setExpiryDate(FilterWarrantExpiryDate[] expiryDate) {
        this.expiryDate = expiryDate;
        return this;
    }

    public QueryWarrantOptions setPriceType(FilterWarrantInOutBoundsType[] priceType) {
        this.priceType = priceType;
        return this;
    }

    public QueryWarrantOptions setStatus(WarrantStatus[] status) {
        this.status = status;
        return this;
    }

}
