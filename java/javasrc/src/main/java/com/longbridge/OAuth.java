package com.longbridge;

import java.util.concurrent.CompletableFuture;

/**
 * OAuth 2.0 client handle for Longbridge OpenAPI
 *
 * <p>
 * Instances are created by {@link OAuthBuilder#build}. This class is an
 * opaque handle to the native OAuth object. Call {@link #close()} (or use
 * try-with-resources) to release native memory when no longer needed.
 *
 * <pre>{@code
 * OAuthBuilder builder = new OAuthBuilder("your-client-id");
 * builder.setCallbackPort(8080);  // optional
 * OAuth oauth = builder.build(url -> System.out.println("Open: " + url)).get();
 * try {
 *     Config config = Config.fromOAuth(oauth);
 * } finally {
 *     oauth.close();
 * }
 * }</pre>
 */
public class OAuth implements AutoCloseable {
    /**
     * @hidden
     */
    long raw;

    /**
     * @hidden
     */
    OAuth(long raw) {
        this.raw = raw;
    }

    /**
     * Returns the raw native pointer for use by other SDK classes.
     *
     * @hidden
     * @return raw native pointer
     */
    long getRaw() {
        long r = this.raw;
        if (r == 0) {
            throw new IllegalStateException("OAuth has already been closed");
        }
        return r;
    }

    /**
     * Revoke the current token on the server (RFC 7009).
     *
     * <p>
     * The refresh token is revoked when present, otherwise the access token.
     * Only the server is contacted; the cached token on disk is left untouched
     * (see {@link #clear()}).
     *
     * @return A Future that completes when the token has been revoked
     */
    public CompletableFuture<Void> revoke() {
        long r = getRaw();
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthRevoke(r, callback);
        });
    }

    /**
     * Clear the token locally.
     *
     * <p>
     * Drops the in-memory token and deletes the cached token file
     * ({@code ~/.longbridge/openapi/tokens/<clientId>}). The server is not
     * contacted (see {@link #revoke()}).
     *
     * @return A Future that completes when the local token has been cleared
     */
    public CompletableFuture<Void> clear() {
        long r = getRaw();
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthClear(r, callback);
        });
    }

    @Override
    public synchronized void close() {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeOAuth(h);
        }
    }
}
