package com.longbridge.quote;

import java.util.Arrays;

public class WatchListGroup {
    private long id;
    private String name;
    private WatchListSecurity[] securities;

    public long getId() {
        return id;
    }

    public String getName() {
        return name;
    }

    public WatchListSecurity[] getSecurities() {
        return securities;
    }

    @Override
    public String toString() {
        return "WatchListGroup [id=" + id + ", name=" + name + ", securities=" + Arrays.toString(securities) + "]";
    }
}
