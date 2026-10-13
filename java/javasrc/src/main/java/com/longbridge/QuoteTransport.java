package com.longbridge;

/**
 * Transport used by the quote pull APIs
 */
public enum QuoteTransport {
    /**
     * Send pull requests over the quote WebSocket connection (default)
     */
    WebSocket,
    /**
     * Send pull requests over HTTP ({@code POST /quote/*}) where the API has a
     * REST equivalent, falling back to the WebSocket for the rest. Using only
     * HTTP-backed APIs never opens a WebSocket connection.
     */
    Http,
}
