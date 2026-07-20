package com.longbridge.agent;

/**
 * An event type not recognized by this SDK version, carried as raw JSON text
 * so callers aren't broken by future additions to the API.
 */
public final class OtherEvent extends ConversationStreamEvent {
    private String json;

    /**
     * Returns the raw event payload as JSON text.
     *
     * @return raw event payload (JSON text)
     */
    public String getJson() {
        return json;
    }

    @Override
    public String toString() {
        return "OtherEvent [json=" + json + "]";
    }
}
