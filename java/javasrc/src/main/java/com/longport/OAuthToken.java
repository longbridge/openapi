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
     * The access token for API authentication
     *
     * @return access token string
     */
    public String getAccessToken() {
        return SdkNative.oauthTokenGetAccessToken(this.raw);
    }

    /**
     * Refresh token, or {@code null} if not provided by the server
     *
     * @return refresh token string, or {@code null}
     */
    public String getRefreshToken() {
        return SdkNative.oauthTokenGetRefreshToken(this.raw);
    }

    /**
     * Unix timestamp (seconds) when the token expires
     *
     * @return expiry time as a Unix timestamp
     */
    public long getExpiresAt() {
        return SdkNative.oauthTokenGetExpiresAt(this.raw);
    }

    /**
     * Returns {@code true} if the token has expired
     *
     * @return whether the token has expired
     */
    public boolean isExpired() {
        return System.currentTimeMillis() / 1000L >= getExpiresAt();
    }

    /**
     * Returns {@code true} if the token will expire within 1 hour
     *
     * @return whether the token expires soon
     */
    public boolean expiresSoon() {
        long now = System.currentTimeMillis() / 1000L;
        long expiresAt = getExpiresAt();
        return expiresAt <= now || (expiresAt - now) < 3600;
    }

    @Override
    public void close() {
        SdkNative.freeOAuthToken(this.raw);
    }

    @Override
    public String toString() {
        return "OAuthToken{accessToken='" + getAccessToken() + "', refreshToken='" + getRefreshToken()
                + "', expiresAt=" + getExpiresAt() + "}";
    }
}
