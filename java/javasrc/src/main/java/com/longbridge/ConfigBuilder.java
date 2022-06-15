package com.longbridge;

public class ConfigBuilder {
    private String appKey;
    private String appSecret;
    private String accessToken;
    private String httpUrl;
    private String quoteWsUrl;
    private String tradeWsUrl;

    public ConfigBuilder(String appKey, String appSecret, String accessToken) {
        this.appKey = appKey;
        this.appSecret = appSecret;
        this.accessToken = accessToken;
    }

    public ConfigBuilder HttpUrl(String httpUrl) {
        this.httpUrl = httpUrl;
        return this;
    }

    public ConfigBuilder quoteWebsocketUrl(String quoteWsUrl) {
        this.quoteWsUrl = quoteWsUrl;
        return this;
    }

    public ConfigBuilder tradeWebsocketUrl(String tradeWsUrl) {
        this.tradeWsUrl = tradeWsUrl;
        return this;
    }

    public Config build() throws OpenApiException {
        return new Config(SdkNative.newConfig(appKey, appSecret, accessToken, httpUrl, quoteWsUrl, tradeWsUrl));
    }
}
