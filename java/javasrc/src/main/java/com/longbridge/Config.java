package com.longbridge;

import java.time.OffsetDateTime;
import java.util.concurrent.CompletableFuture;

/**
 * Configuration options for Longbridge SDK
 */
public class Config implements AutoCloseable {
    private long raw;

    /**
     * @hidden
     */
    Config(long config) {
        this.raw = config;
    }

    /**
     * Create a new {@code Config} from API key credentials.
     * <p>
     * Optional environment variables are read automatically:
     * {@code LONGBRIDGE_HTTP_URL}, {@code LONGBRIDGE_LANGUAGE},
     * {@code LONGBRIDGE_QUOTE_WS_URL}, {@code LONGBRIDGE_TRADE_WS_URL},
     * {@code LONGBRIDGE_ENABLE_OVERNIGHT}, {@code LONGBRIDGE_PUSH_CANDLESTICK_MODE},
     * {@code LONGBRIDGE_PRINT_QUOTE_PACKAGES}, {@code LONGBRIDGE_LOG_PATH}.
     * Use the chainable setter methods (e.g. {@link #httpUrl}) to override any of
     * these values.
     *
     * @param appKey      App key
     * @param appSecret   App secret
     * @param accessToken Access token
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromApikey(String appKey, String appSecret, String accessToken) throws OpenApiException {
        return new Config(SdkNative.newConfigFromApikey(appKey, appSecret, accessToken));
    }

    /**
     * Create a new {@code Config} from the given environment variables
     * <p>
     * It first gets the environment variables from the .env file in the current
     * directory.
     *
     * <p><b>Variables</b></p>
     * <ul>
     * <li>{@code LONGBRIDGE_LANGUAGE} - Language identifier, {@code zh-CN},
     * {@code zh-HK} or {@code en} (Default: {@code en})</li>
     * <li>{@code LONGBRIDGE_APP_KEY} - App key</li>
     * <li>{@code LONGBRIDGE_APP_SECRET} - App secret</li>
     * <li>{@code LONGBRIDGE_ACCESS_TOKEN} - Access token</li>
     * <li>{@code LONGBRIDGE_HTTP_URL} - HTTP endpoint url (Default:
     * {@code https://openapi.longbridge.com})</li>
     * <li>{@code LONGBRIDGE_QUOTE_WS_URL} - Quote websocket endpoint url (Default:
     * {@code wss://openapi-quote.longbridge.com/v2})</li>
     * <li>{@code LONGBRIDGE_TRADE_WS_URL} - Trade websocket endpoint url (Default:
     * {@code wss://openapi-trade.longbridge.com/v2})</li>
     * <li>{@code LONGBRIDGE_ENABLE_OVERNIGHT} - Enable overnight quote, {@code true}
     * or {@code false} (Default: {@code false})</li>
     * <li>{@code LONGBRIDGE_PUSH_CANDLESTICK_MODE} - {@code realtime} or
     * {@code confirmed} (Default: {@code realtime})</li>
     * <li>{@code LONGBRIDGE_PRINT_QUOTE_PACKAGES} - Print quote packages when
     * connected, {@code true} or {@code false} (Default: {@code true})</li>
     * <li>{@code LONGBRIDGE_LOG_PATH} - Set the path of the log files (Default: no
     * logs)</li>
     * </ul>
     *
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromApikeyEnv() throws OpenApiException {
        return new Config(SdkNative.newConfigFromApikeyEnv());
    }

    /**
     * Create a new {@code Config} for OAuth 2.0 authentication.
     * <p>
     * OAuth 2.0 is the recommended authentication method. Obtain an {@link OAuth}
     * instance via {@link OAuthBuilder#build}.
     * <p>
     * Optional environment variables are read automatically:
     * {@code LONGBRIDGE_HTTP_URL}, {@code LONGBRIDGE_LANGUAGE},
     * {@code LONGBRIDGE_QUOTE_WS_URL}, {@code LONGBRIDGE_TRADE_WS_URL},
     * {@code LONGBRIDGE_ENABLE_OVERNIGHT}, {@code LONGBRIDGE_PUSH_CANDLESTICK_MODE},
     * {@code LONGBRIDGE_PRINT_QUOTE_PACKAGES}, {@code LONGBRIDGE_LOG_PATH}.
     * Use the chainable setter methods (e.g. {@link #httpUrl}) to override any of
     * these values.
     *
     * @param oauth OAuth handle returned by {@link OAuthBuilder#build}
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromOAuth(OAuth oauth) throws OpenApiException {
        return new Config(SdkNative.newConfigFromOauth(oauth.getRaw()));
    }

    /**
     * Set the HTTP endpoint URL.
     * <p>
     * NOTE: Usually you don't need to change it.
     *
     * @param httpUrl OpenAPI endpoint (Default: {@code https://openapi.longbridge.com})
     * @return this object
     */
    public Config httpUrl(String httpUrl) {
        this.raw = SdkNative.configSetHttpUrl(this.raw, httpUrl);
        return this;
    }

    /**
     * Set the quote websocket endpoint URL.
     * <p>
     * NOTE: Usually you don't need to change it.
     *
     * @param quoteWsUrl OpenAPI quote websocket endpoint
     * @return this object
     */
    public Config quoteWebsocketUrl(String quoteWsUrl) {
        this.raw = SdkNative.configSetQuoteWsUrl(this.raw, quoteWsUrl);
        return this;
    }

    /**
     * Set the trade websocket endpoint URL.
     * <p>
     * NOTE: Usually you don't need to change it.
     *
     * @param tradeWsUrl OpenAPI trade websocket endpoint
     * @return this object
     */
    public Config tradeWebsocketUrl(String tradeWsUrl) {
        this.raw = SdkNative.configSetTradeWsUrl(this.raw, tradeWsUrl);
        return this;
    }

    /**
     * Set the language identifier.
     *
     * @param language Language identifier (Default: {@link Language#EN})
     * @return this object
     */
    public Config language(Language language) {
        this.raw = SdkNative.configSetLanguage(this.raw, language);
        return this;
    }

    /**
     * Enable overnight quote.
     *
     * @return this object
     */
    public Config enableOvernight() {
        this.raw = SdkNative.configSetEnableOvernight(this.raw);
        return this;
    }

    /**
     * Set the push candlestick mode.
     *
     * @param mode Mode (Default: {@link PushCandlestickMode#Realtime})
     * @return this object
     */
    public Config pushCandlestickMode(PushCandlestickMode mode) {
        this.raw = SdkNative.configSetPushCandlestickMode(this.raw, mode);
        return this;
    }

    /**
     * Disable printing quote packages when connected to the server.
     *
     * @return this object
     */
    public Config disablePrintQuotePackages() {
        this.raw = SdkNative.configSetEnablePrintQuotePackages(this.raw, false);
        return this;
    }

    /**
     * Set the path of the log files.
     *
     * @param path The path of the log files (Default: no logs)
     * @return this object
     */
    public Config logPath(String path) {
        this.raw = SdkNative.configSetLogPath(this.raw, path);
        return this;
    }

    /**
     * Enable paper trading mode.
     *
     * <p>When enabled, all API calls target the paper trading (simulation)
     * environment.  The server validates the token: if it belongs to a real-money
     * account the server returns an error.
     *
     * <p>By default this option is disabled and the server imposes no restrictions
     * — requests from both paper trading and real-money accounts are accepted.
     *
     * <p>Paper trading users should call this method as a safety guard.
     *
     * @return this object
     */
    public Config enablePapertrading() {
        this.raw = SdkNative.configSetEnablePapertrading(this.raw);
        return this;
    }

    /**
     * Gets a new {@code access_token}.
     *
     * <p>This method is only available when using <b>Legacy API Key</b>
     * authentication (i.e. {@link #fromApikey}). It is not supported for OAuth
     * 2.0 mode.
     *
     * @param expiredAt The expiration time of the new access token. Pass
     *                  {@code null} to use the default (90 days from now).
     * @return A {@link CompletableFuture} that resolves to the new access token
     *         string.
     * @see <a href="https://open.longportapp.com/en/docs/refresh-token-api">Refresh Token API</a>
     */
    public CompletableFuture<String> refreshAccessToken(OffsetDateTime expiredAt) {
        CompletableFuture<String> future = new CompletableFuture<>();
        SdkNative.configRefreshAccessToken(this.raw, expiredAt, new AsyncCallback() {
            @Override
            public void callback(Object err, Object result) {
                if (err != null) {
                    future.completeExceptionally((OpenApiException) err);
                } else {
                    future.complete((String) result);
                }
            }
        });
        return future;
    }

    /**
     * @hidden
     * @return Context pointer
     */
    public long getRaw() {
        return this.raw;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeConfig(this.raw);
    }
}
