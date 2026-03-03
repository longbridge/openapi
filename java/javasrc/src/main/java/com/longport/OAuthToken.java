package com.longport;

/**
 * OAuth 2.0 access token (opaque native handle)
 *
 * <p>
 * Instances are returned by {@link OAuth#authorize} and {@link OAuth#refresh}.
 * Call {@link #close()} (or use try-with-resources) to release native memory
 * when the token is no longer needed.
 */
public class OAuthToken implements AutoCloseable {
    /**
     * @hidden
     */
    long raw;

    /**
     * @hidden
     */
    public OAuthToken() {
    }

    /**
     * Returns {@code true} if the token has expired
     *
     * @return whether the token has expired
     */
    public boolean isExpired() {
        return SdkNative.oauthTokenIsExpired(this.raw);
    }

    /**
     * Returns {@code true} if the token will expire within 1 hour
     *
     * @return whether the token expires soon
     */
    public boolean expiresSoon() {
        return SdkNative.oauthTokenExpiresSoon(this.raw);
    }

    @Override
    public void close() {
        SdkNative.freeOAuthToken(this.raw);
    }

    @Override
    public String toString() {
        return "OAuthToken{expired=" + isExpired() + ", expiresSoon=" + expiresSoon() + "}";
    }
}
