package com.longport.quote;

@SuppressWarnings("unused")
public class CreateWatchlistGroup {
    private String name;
    private String[] securities;

    public CreateWatchlistGroup(String name) {
        this.name = name;
    }

    public CreateWatchlistGroup setSecurities(String[] securities) {
        this.securities = securities;
        return this;
    }
}
