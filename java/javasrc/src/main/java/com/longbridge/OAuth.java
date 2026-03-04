package com.longbridge;

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
    final long raw;

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
        return this.raw;
    }

    @Override
    public void close() {
        SdkNative.freeOAuth(this.raw);
    }
}
