package com.longbridge;

/**
 * A Config object builder
 */
public class ConfigBuilder {
    private String appKey;
    private String appSecret;
    private String accessToken;
    private String httpUrl;
    private String quoteWsUrl;
    private String tradeWsUrl;

    /**
     * Create a `Config` object builder
     * 
     * @param appKey      App Key
     * @param appSecret   App Secret
     * @param accessToken Access Token
     */
    public ConfigBuilder(String appKey, String appSecret, String accessToken) {
        this.appKey = appKey;
        this.appSecret = appSecret;
        this.accessToken = accessToken;
    }

    /**
     * Specifies the url of the OpenAPI server.
     * <p>
     * NOTE: Usually you don’t need to change it.
     * 
     * @param httpUrl OpenAPI endpoint (Default:
     *                `https://openapi.longbridgeapp.com`)
     * @return this object
     */
    public ConfigBuilder HttpUrl(String httpUrl) {
        this.httpUrl = httpUrl;
        return this;
    }

    /**
     * Specifies the url of the OpenAPI quote websocket server.
     * <p>
     * NOTE: Usually you don’t need to change it.
     * 
     * @param quoteWsUrl OpenAPI quote websocket endpoint (Default:
     *                   `wss://openapi-quote.longbridgeapp.com`)
     * @return this object
     */
    public ConfigBuilder quoteWebsocketUrl(String quoteWsUrl) {
        this.quoteWsUrl = quoteWsUrl;
        return this;
    }

    /**
     * Specifies the url of the OpenAPI trade websocket server.
     * <p>
     * NOTE: Usually you don’t need to change it.
     * 
     * @param tradeWsUrl OpenAPI trade websocket endpoint (Default:
     *                   `wss://openapi-quote.longbridgeapp.com`)
     * @return this object
     */
    public ConfigBuilder tradeWebsocketUrl(String tradeWsUrl) {
        this.tradeWsUrl = tradeWsUrl;
        return this;
    }

    /**
     * Build a Config object
     * 
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public Config build() throws OpenApiException {
        return new Config(SdkNative.newConfig(appKey, appSecret, accessToken, httpUrl, quoteWsUrl, tradeWsUrl));
    }
}
