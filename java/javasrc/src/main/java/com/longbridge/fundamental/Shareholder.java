package com.longbridge.fundamental;

import java.math.BigDecimal;

/** One major shareholder of a security. */
public class Shareholder {
    /** Internal shareholder ID (string form). */
    public String shareholderId;
    /** Shareholder name. */
    public String shareholderName;
    /** Institution type (may be empty). */
    public String institutionType;
    /** Percentage of shares held. */
    public BigDecimal percentOfShares;
    /** Change in shares held (positive = bought, negative = sold). */
    public BigDecimal sharesChanged;
    /** Date of the most recent filing, e.g. {@code "2026-05-04"}. */
    public String reportDate;
    /** Other securities held by this shareholder (cross-holdings). */
    public ShareholderStock[] stocks;
}