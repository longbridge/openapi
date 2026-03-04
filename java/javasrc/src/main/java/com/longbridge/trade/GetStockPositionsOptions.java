package com.longbridge.trade;

@SuppressWarnings("unused")
public class GetStockPositionsOptions {
    private String[] symbols;

    public GetStockPositionsOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }
}
