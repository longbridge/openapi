package com.longport.trade;

@SuppressWarnings("unused")
public class GetFundPositionsOptions {
    private String[] symbols;

    public GetFundPositionsOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }
}
