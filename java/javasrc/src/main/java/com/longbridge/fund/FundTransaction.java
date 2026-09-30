package com.longbridge.fund;

/**
 * FundTransaction
 */
public class FundTransaction {
    private String amount;
    private String category;
    private long createdAt;
    private String currency;
    private String description;
    private long detailCreatedAt;
    private String detailType;
    private long doneAt;
    private String quantityDescription;
    private String redirectPage;
    private String redirectPageV2;
    private String refNo;
    private String stockQuantity;
    private String txType;
    private String typeName;

    /**
     * Returns amount.
     *
     * @return amount
     */
    public String getAmount() {
        return amount;
    }

    /**
     * Returns category.
     *
     * @return category
     */
    public String getCategory() {
        return category;
    }

    /**
     * Returns createdAt.
     *
     * @return createdAt
     */
    public long getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns description.
     *
     * @return description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns detailCreatedAt.
     *
     * @return detailCreatedAt
     */
    public long getDetailCreatedAt() {
        return detailCreatedAt;
    }

    /**
     * Returns detailType.
     *
     * @return detailType
     */
    public String getDetailType() {
        return detailType;
    }

    /**
     * Returns doneAt.
     *
     * @return doneAt
     */
    public long getDoneAt() {
        return doneAt;
    }

    /**
     * Returns quantityDescription.
     *
     * @return quantityDescription
     */
    public String getQuantityDescription() {
        return quantityDescription;
    }

    /**
     * Returns redirectPage.
     *
     * @return redirectPage
     */
    public String getRedirectPage() {
        return redirectPage;
    }

    /**
     * Returns redirectPageV2.
     *
     * @return redirectPageV2
     */
    public String getRedirectPageV2() {
        return redirectPageV2;
    }

    /**
     * Returns refNo.
     *
     * @return refNo
     */
    public String getRefNo() {
        return refNo;
    }

    /**
     * Returns stockQuantity.
     *
     * @return stockQuantity
     */
    public String getStockQuantity() {
        return stockQuantity;
    }

    /**
     * Returns txType.
     *
     * @return txType
     */
    public String getTxType() {
        return txType;
    }

    /**
     * Returns typeName.
     *
     * @return typeName
     */
    public String getTypeName() {
        return typeName;
    }

    @Override
    public String toString() {
        return                 "FundTransaction [amount=" + amount +
                ", category=" + category +
                ", createdAt=" + createdAt +
                ", currency=" + currency +
                ", description=" + description +
                ", detailCreatedAt=" + detailCreatedAt +
                ", detailType=" + detailType +
                ", doneAt=" + doneAt +
                ", quantityDescription=" + quantityDescription +
                ", redirectPage=" + redirectPage +
                ", redirectPageV2=" + redirectPageV2 +
                ", refNo=" + refNo +
                ", stockQuantity=" + stockQuantity +
                ", txType=" + txType +
                ", typeName=" + typeName + "]";
    }
}
