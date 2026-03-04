package com.longbridge.quote;

@SuppressWarnings("unused")
public class DeleteWatchlistGroup {
    private long id;
    private boolean purge;

    public DeleteWatchlistGroup(long id) {
        this.id = id;
        this.purge = false;
    }

    public DeleteWatchlistGroup purge() {
        this.purge = true;
        return this;
    }
}
