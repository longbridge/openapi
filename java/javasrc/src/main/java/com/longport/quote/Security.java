package com.longport.quote;

public class Security {
    private String symbol;
    private String nameCn;
    private String nameEn;
    private String nameHk;

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

    @Override
    public String toString() {
        return "Security [symbol=" + symbol + ", nameCn=" + nameCn + ", nameEn=" + nameEn + ", nameHk=" + nameHk + "]";
    }
}