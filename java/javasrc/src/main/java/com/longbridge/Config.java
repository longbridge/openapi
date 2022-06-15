package com.longbridge;

public class Config implements AutoCloseable {
    private long raw;

    Config(long config) {
        this.raw = config;
    }

    public static Config fromEnv() throws OpenApiException {
        return new Config(SdkNative.newConfigFromEnv());
    }

    public long getRaw() {
        return this.raw;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeConfig(this.raw);
    }
}
