package com.longport;

/**
 * Configuration options for LongPort sdk
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
     * Create a new {@code Config} from the given environment variables
     * <p>
     * It first gets the environment variables from the .env file in the current
     * directory.
     *
     * <h4>Variables</h4>
     * <ul>
     * <li>{@code LONGPORT_LANGUAGE} - Language identifier, {@code zh-CN},
     * {@code zh-HK} or {@code en} (Default: {@code en})</li>
     * <li>{@code LONGPORT_APP_KEY} - App key</li>
     * <li>{@code LONGPORT_APP_SECRET} - App secret</li>
     * <li>{@code LONGPORT_ACCESS_TOKEN} - Access token</li>
     * <li>{@code LONGPORT_HTTP_URL} - HTTP endpoint url (Default:
     * {@code https://openapi.longportapp.com})</li>
     * <li>{@code LONGPORT_QUOTE_WS_URL} - Quote websocket endpoint url (Default:
     * {@code wss://openapi-quote.longportapp.com/v2})</li>
     * <li>{@code LONGPORT_TRADE_WS_URL} - Trade websocket endpoint url (Default:
     * {@code wss://openapi-trade.longportapp.com/v2})</li>
     * <li>{@code LONGPORT_ENABLE_OVERNIGHT} - Enable overnight quote, {@code true}
     * or {@code false} (Default: {@code false})</li>
     * <li>{@code LONGPORT_PUSH_CANDLESTICK_MODE} - {@code realtime} or
     * {@code confirmed} (Default: {@code realtime})</li>
     * <li>{@code LONGPORT_PRINT_QUOTE_PACKAGES} - Print quote packages when
     * connected, {@code true} or {@code false} (Default: {@code true})</li>
     * <li>{@code LONGPORT_LOG_PATH} - Set the path of the log files (Default: no
     * logs)</li>
     * </ul>
     *
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromEnv() throws OpenApiException {
        return new Config(SdkNative.newConfigFromEnv());
    }

    /**
     * Create a new {@code Config} for OAuth 2.0 authentication.
     * <p>
     * OAuth 2.0 is the recommended authentication method. Obtain an {@link OAuth}
     * instance via {@link OAuthBuilder#build}.
     *
     * @param oauth OAuth handle returned by {@link OAuthBuilder#build}
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromOAuth(OAuth oauth) throws OpenApiException {
        return new Config(SdkNative.newConfigFromOauth(oauth.getRaw()));
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
