package com.longbridge;

/**
 * Configuration options for Longbridge sdk
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
}
