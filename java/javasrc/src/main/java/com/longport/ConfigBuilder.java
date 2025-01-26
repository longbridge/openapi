package com.longport;

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
    private Language language;
    private boolean enableOvernight;
    private PushCandlestickMode pushCandlestickMode;
    private boolean enablePrintQuotePackages;
    private String logPath;

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
        this.enablePrintQuotePackages = true;
    }

    /**
     * Specifies the url of the OpenAPI server.
     * <p>
     * NOTE: Usually you don’t need to change it.
     * 
     * @param httpUrl OpenAPI endpoint (Default:
     *                `https://openapi.longportapp.com`)
     * @return this object
     */
    public ConfigBuilder httpUrl(String httpUrl) {
        this.httpUrl = httpUrl;
        return this;
    }

    /**
     * Specifies the url of the OpenAPI quote websocket server.
     * <p>
     * NOTE: Usually you don’t need to change it.
     * 
     * @param quoteWsUrl OpenAPI quote websocket endpoint (Default:
     *                   `wss://openapi-quote.longportapp.com`)
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
     *                   `wss://openapi-quote.longportapp.com`)
     * @return this object
     */
    public ConfigBuilder tradeWebsocketUrl(String tradeWsUrl) {
        this.tradeWsUrl = tradeWsUrl;
        return this;
    }

    /**
     * Specifies the language identifer
     * 
     * @param language Language identifer (Default: Language.EN)
     * @return this object
     */
    public ConfigBuilder language(Language language) {
        this.language = language;
        return this;
    }

    /**
     * Enable overnight quote
     * 
     * @return this object
     */
    public ConfigBuilder enableOvernight() {
        this.enableOvernight = true;
        return this;
    }

    /**
     * Specifies the push candlestick mode
     * 
     * @param mode Mode (Default: PushCandlestickMode.Realtime)
     * @return this object
     */
    public ConfigBuilder pushCandlestickMode(PushCandlestickMode mode) {
        this.pushCandlestickMode = mode;
        return this;
    }

    /**
     * Don't print quote packages when connected to the server
     * 
     * @return this object
     */
    public ConfigBuilder dontPrintQuotePackages() {
        this.enablePrintQuotePackages = false;
        return this;
    }

    /**
     * Set the path of the log files.
     * 
     * @param path The path of the log files (Default: `no logs`)
     * @return this object
     */
    public ConfigBuilder logPath(String path) {
        this.logPath = path;
        return this;
    }

    /**
     * Build a Config object
     * 
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public Config build() throws OpenApiException {
        return new Config(
                SdkNative.newConfig(appKey, appSecret, accessToken, httpUrl, quoteWsUrl, tradeWsUrl, language,
                        enableOvernight, pushCandlestickMode, enablePrintQuotePackages, logPath));
    }
}
