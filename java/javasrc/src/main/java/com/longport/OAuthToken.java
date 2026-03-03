package com.longport;

import java.util.concurrent.CompletableFuture;

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

    /**
     * Load a token from the default path ({@code ~/.longbridge-openapi/token})
     *
     * @return CompletableFuture that resolves to an {@link OAuthToken}
     */
    public static CompletableFuture<OAuthToken> load() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthTokenLoad(callback);
        });
    }

    /**
     * Load a token from an explicit file path
     *
     * @param path Path to the token JSON file
     * @return CompletableFuture that resolves to an {@link OAuthToken}
     */
    public static CompletableFuture<OAuthToken> loadFromPath(String path) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthTokenLoadFromPath(path, callback);
        });
    }

    /**
     * Save the token to the default path ({@code ~/.longbridge-openapi/token})
     *
     * <p>
     * The parent directory is created automatically if it does not exist.
     *
     * @return CompletableFuture that completes when the file is written
     */
    public CompletableFuture<Void> save() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthTokenSave(this.raw, callback);
        });
    }

    /**
     * Save the token to an explicit file path
     *
     * <p>
     * The parent directory is created automatically if it does not exist.
     *
     * @param path Destination path for the token JSON file
     * @return CompletableFuture that completes when the file is written
     */
    public CompletableFuture<Void> saveToPath(String path) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthTokenSaveToPath(this.raw, path, callback);
        });
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
