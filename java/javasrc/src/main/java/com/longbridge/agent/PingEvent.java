package com.longbridge.agent;

/**
 * A heartbeat with no payload, observed at arbitrary points in the stream
 * (including in between {@link MessageEvent} chunks).
 */
public final class PingEvent extends ConversationStreamEvent {
    @Override
    public String toString() {
        return "PingEvent []";
    }
}
