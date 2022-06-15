package com.longbridge.quote;

import java.math.BigDecimal;

public class SecurityStaticInfo {
    private String symbol;
    private String nameCn;
    private String nameEn;
    private String nameHk;
    private String exchange;
    private String currency;
    private int lotSize;
    private long totalShares;
    private long circulatingShares;
    private long hkShares;
    private BigDecimal eps;
    private BigDecimal epsTtm;
    private BigDecimal bps;
    private BigDecimal dividendYield;
    private DerivativeType[] stockDerivatives;

    public String getSymbol() {
        return symbol;
    }

    public String getNameCn() {
        return nameCn;
    }

    public String getNameEn() {
        return nameEn;
    }

    public String getNameHk() {
        return nameHk;
    }

    public String getExchange() {
        return exchange;
    }

    public String getCurrency() {
        return currency;
    }

    public int getLotSize() {
        return lotSize;
    }

    public long getTotalShares() {
        return totalShares;
    }

    public long getCirculatingShares() {
        return circulatingShares;
    }

    public long getHkShares() {
        return hkShares;
    }

    public BigDecimal getEps() {
        return eps;
    }

    public BigDecimal getEpsTtm() {
        return epsTtm;
    }

    public BigDecimal getBps() {
        return bps;
    }

    public BigDecimal getDividendYield() {
        return dividendYield;
    }

    public DerivativeType[] getStockDerivatives() {
        return stockDerivatives;
    }
}
