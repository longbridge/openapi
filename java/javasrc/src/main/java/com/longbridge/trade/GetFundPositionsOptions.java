package com.longbridge.trade;

@SuppressWarnings("unused")
public class GetFundPositionsOptions {
    private String[] symbols;

    public GetFundPositionsOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }
}
