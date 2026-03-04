package com.longbridge;

import java.util.concurrent.CompletableFuture;
import java.util.function.Consumer;

/**
 * Builder for constructing an {@link OAuth} client
 *
 * <p>
 * {@code clientId} is the only required parameter. Optionally set a custom
 * callback port before calling {@link #build}.
 *
 * <p>
 * The builder will attempt to load an existing token from
 * {@code ~/.longbridge-openapi/tokens/<clientId>}. If no valid token is found,
 * the full browser-based authorization flow is started and {@code onOpenUrl}
 * is called with the authorization URL. The resulting token is persisted for
 * future use.
 *
 * <pre>{@code
 * OAuthBuilder builder = new OAuthBuilder("your-client-id");
 * builder.setCallbackPort(8080);  // optional, default 60355
 * OAuth oauth = builder.build(url -> {
 *     System.out.println("Open this URL: " + url);
 * }).get();
 * try {
 *     Config config = Config.fromOAuth(oauth);
 * } finally {
 *     oauth.close();
 * }
 * }</pre>
 */
public class OAuthBuilder {
    private final String clientId;
    private int callbackPort;

    /**
     * Create a new {@code OAuthBuilder} with the given client ID.
     *
     * @param clientId OAuth 2.0 client ID from the Longbridge developer portal
     */
    public OAuthBuilder(String clientId) {
        this.clientId = clientId;
        this.callbackPort = 0; // 0 means use the default (60355)
    }

    /**
     * Set the local callback server port.
     *
     * <p>
     * Must match one of the redirect URIs registered for the client.
     * Defaults to {@code 60355} if not set (or set to {@code 0}).
     *
     * @param port TCP port for the local callback server
     * @return this builder
     */
    public OAuthBuilder setCallbackPort(int port) {
        this.callbackPort = port;
        return this;
    }

    /**
     * Asynchronously build the {@link OAuth} client.
     *
     * <p>
     * If a valid token already exists on disk it is loaded directly; otherwise
     * {@code onOpenUrl} is invoked with the authorization URL and the full
     * browser-based flow is started.
     *
     * @param onOpenUrl Called with the authorization URL; open it in a browser
     *                  or print it to the console
     * @return CompletableFuture that resolves to a new {@link OAuth} handle
     */
    public CompletableFuture<OAuth> build(Consumer<String> onOpenUrl) {
        return AsyncCallback.<Long>executeTask((callback) -> {
            SdkNative.oauthBuild(this.clientId, this.callbackPort, onOpenUrl, callback);
        }).thenApply(raw -> new OAuth(raw));
    }
}
