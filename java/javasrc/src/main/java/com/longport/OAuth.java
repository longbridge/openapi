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
 *     Config config = Config.fromOAuth(token);
 * }
 * }</pre>
 */
public class OAuth implements AutoCloseable {
    private long raw;

    /**
     * Create a new OAuth 2.0 client with the default callback port (60355)
     *
     * @param clientId OAuth 2.0 client ID from the LongPort developer portal
     */
    public OAuth(String clientId) {
        this.raw = SdkNative.newOAuth(clientId, 60355);
    }

    /**
     * Create a new OAuth 2.0 client with a custom callback port
     *
     * @param clientId     OAuth 2.0 client ID from the LongPort developer portal
     * @param callbackPort TCP port for the local callback server. Must match one
     *                     of the redirect URIs registered for the client.
     */
    public OAuth(String clientId, int callbackPort) {
        this.raw = SdkNative.newOAuth(clientId, callbackPort);
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
     * Refresh an access token using an existing OAuthToken
     *
     * @param token Existing OAuthToken whose refresh token is used
     * @return CompletableFuture that resolves to a new {@link OAuthToken}
     */
    public CompletableFuture<OAuthToken> refresh(OAuthToken token) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.oauthRefresh(this.raw, token.raw, callback);
        });
    }

    @Override
    public void close() {
        SdkNative.freeOAuth(this.raw);
    }
}
