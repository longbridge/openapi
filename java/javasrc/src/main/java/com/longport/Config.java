package com.longport;

import java.time.OffsetDateTime;
import java.util.concurrent.CompletableFuture;

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
     * Create a new `Config` from the given environment variables
     * <p>
     * It first gets the environment variables from the .env file in the current
     * directory.
     * 
     * # Variables
     * 
     * - `LONGPORT_APP_KEY` - App key
     * - `LONGPORT_APP_SECRET` - App secret
     * - `LONGPORT_ACCESS_TOKEN` - Access token
     * - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default:
     * `https://openapi.longportapp.com`)
     * - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
     * `wss://openapi-quote.longportapp.com/v2`)
     * - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
     * `wss://openapi-trade.longportapp.com/v2`)
     * - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
     * `false` (Default: `false`)
     * - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
     * `realtime`)
     * 
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static Config fromEnv() throws OpenApiException {
        return new Config(SdkNative.newConfigFromEnv());
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

    public CompletableFuture<String> refreshAccessToken(OffsetDateTime expiredAt) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.configRefreshAccessToken(this.raw, expiredAt, callback);
        });
    }
}
