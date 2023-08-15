package com.longbridge.quote;

import java.util.Arrays;

public class WatchlistGroup {
    private long id;
    private String name;
    private WatchlistSecurity[] securities;

    public long getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public WatchlistSecurity[] getSecurities() {
        return securities;
    }

    @Override
    public String toString() {
        return "WatchlistGroup [id=" + id + ", name=" + name + ", securities=" + Arrays.toString(securities) + "]";
    }
}
