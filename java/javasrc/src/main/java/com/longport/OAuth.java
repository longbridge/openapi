package com.longport;

import java.util.concurrent.CompletableFuture;
import java.util.function.Consumer;

/**
 * OAuth 2.0 client for LongPort OpenAPI
 *
 * <p>
 * Use this class to perform the OAuth 2.0 authorization code flow and obtain
 * access tokens for API authentication.
 *
 * <pre>{@code
 * OAuth oauth = new OAuth("your-client-id");
 * try (OAuthToken token = oauth.authorize(url -> {
 *     System.out.println("Open this URL: " + url);
 * }).get()) {
 *     Config config = Config.fromOauth(token.getAccessToken());
 * }
 * }</pre>
 */
public class OAuth implements AutoCloseable {
    private long raw;

    /**
     * Create a new OAuth 2.0 client
     *
     * @param clientId OAuth 2.0 client ID from the LongPort developer portal
     */
    public OAuth(String clientId) {
        this.raw = SdkNative.newOAuth(clientId);
    }

    /**
     * Start the OAuth 2.0 authorization flow
     *
     * <p>
     * Starts a local HTTP server, calls {@code onOpenUrl} with the authorization
     * URL, then waits for the redirect and exchanges the code for a token.
     *
     * @param onOpenUrl Called with the authorization URL; open it in a browser or
     *                  print it
     * @return CompletableFuture that resolves to an {@link OAuthToken}
     */
    public CompletableFuture<OAuthToken> authorize(Consumer<String> onOpenUrl) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthAuthorize(this.raw, onOpenUrl, callback);
        });
    }

    /**
     * Refresh an access token using a refresh token
     *
     * @param refreshToken Refresh token from a previous authorization
     * @return CompletableFuture that resolves to a new {@link OAuthToken}
     */
    public CompletableFuture<OAuthToken> refresh(String refreshToken) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthRefresh(this.raw, refreshToken, callback);
        });
    }

    @Override
    public void close() {
        SdkNative.freeOAuth(this.raw);
    }
}
