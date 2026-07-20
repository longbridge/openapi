package com.longbridge.agent;

/**
 * An event type not recognized by this SDK version, carried as raw JSON text
 * so callers aren't broken by future additions to the API.
 */
public final class OtherEvent extends ConversationStreamEvent {
    private String event;
    private String json;

    /**
     * Returns the SSE envelope's {@code event} field (the event type name),
     * e.g. {@code "workflow_started"}, {@code "ping"}, {@code "chat_finished"},
     * {@code "chat_title_updated"} (observed against the real API; not
     * documented).
     *
     * @return event type name
     */
    public String getEvent() {
        return event;
    }

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
        return "OtherEvent [event=" + event + ", json=" + json + "]";
    }
}
