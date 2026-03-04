package com.longbridge.quote;

@SuppressWarnings("unused")
public class UpdateWatchlistGroup {
    private long id;
    private String name;
    private String[] securities;
    private SecuritiesUpdateMode mode;

    public UpdateWatchlistGroup(long id) {
        this.id = id;
        this.mode = SecuritiesUpdateMode.Replace;
    }

    public UpdateWatchlistGroup setName(String name) {
        this.name = name;
        return this;
    }

    public UpdateWatchlistGroup setSecurities(String[] securities) {
        this.securities = securities;
        return this;
    }

    public UpdateWatchlistGroup setMode(SecuritiesUpdateMode mode) {
        this.mode = mode;
        return this;
    }
}
