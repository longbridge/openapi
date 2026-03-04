package com.longbridge;

import java.util.HashMap;
import java.util.concurrent.CompletableFuture;
import com.google.gson.Gson;

public class HttpClient implements AutoCloseable {
    private long raw;

    /**
     * @hidden
     */
    HttpClient(long raw) {
        this.raw = raw;
    }

    /**
     * Create a new {@code HttpClient} using API Key authentication.
     * <p>
     * {@code LONGBRIDGE_HTTP_URL} is read from the environment automatically.
     *
     * @param appKey      App key
     * @param appSecret   App secret
     * @param accessToken Access token
     * @return HttpClient object
     */
    public static HttpClient fromApikey(String appKey, String appSecret, String accessToken) {
        return new HttpClient(SdkNative.newHttpClientFromApikey(appKey, appSecret, accessToken, null));
    }

    /**
     * Create a new {@code HttpClient} using API Key authentication with a custom
     * HTTP endpoint URL.
     * <p>
     * The {@code httpUrl} parameter overrides {@code LONGBRIDGE_HTTP_URL} from the
     * environment.
     *
     * @param appKey      App key
     * @param appSecret   App secret
     * @param accessToken Access token
     * @param httpUrl     HTTP endpoint URL override
     * @return HttpClient object
     */
    public static HttpClient fromApikey(String appKey, String appSecret, String accessToken, String httpUrl) {
        return new HttpClient(SdkNative.newHttpClientFromApikey(appKey, appSecret, accessToken, httpUrl));
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeHttpClient(this.raw);
    }

    /**
     * Create a new {@code HttpClient} from environment variables (API Key
     * authentication).
     * <p>
     * Variables: {@code LONGBRIDGE_HTTP_URL}, {@code LONGBRIDGE_APP_KEY},
     * {@code LONGBRIDGE_APP_SECRET}, {@code LONGBRIDGE_ACCESS_TOKEN}
     *
     * @return HttpClient object
     * @throws OpenApiException If an error occurs
     */
    public static HttpClient fromApikeyEnv() throws OpenApiException {
        return new HttpClient(SdkNative.newHttpClientFromApikeyEnv());
    }

    /**
     * Create a new {@code HttpClient} from an OAuth handle.
     * <p>
     * {@code LONGBRIDGE_HTTP_URL} is read from the environment automatically.
     *
     * @param oauth OAuth handle returned by {@link OAuthBuilder#build}
     * @return HttpClient object
     */
    public static HttpClient fromOAuth(OAuth oauth) {
        return new HttpClient(SdkNative.newHttpClientFromOauth(oauth.getRaw(), null));
    }

    /**
     * Create a new {@code HttpClient} from an OAuth handle with a custom HTTP
     * endpoint URL.
     * <p>
     * The {@code httpUrl} parameter overrides {@code LONGBRIDGE_HTTP_URL} from the
     * environment.
     *
     * @param oauth   OAuth handle returned by {@link OAuthBuilder#build}
     * @param httpUrl HTTP endpoint URL override
     * @return HttpClient object
     */
    public static HttpClient fromOAuth(OAuth oauth, String httpUrl) {
        return new HttpClient(SdkNative.newHttpClientFromOauth(oauth.getRaw(), httpUrl));
    }

    /**
     * Performs a HTTP request
     * 
     * @param <T>       Response class type
     * @param respClass Response class object, it can be null
     * @param method    HTTP method, e.g. get, post
     * @param path      Request path
     * @return A Future representing the result of the operation
     * @throws RuntimeException If an error occurs
     */
    public <T> CompletableFuture<T> request(Class<T> respClass, String method, String path)
            throws RuntimeException {
        return doRequest(respClass, method, path, null, null);
    }

    /**
     * Performs a HTTP request with body
     * 
     * @param <T>         Response class type
     * @param respClass   Response class object, it can be null
     * @param method      HTTP method, e.g. get, post
     * @param path        Request path
     * @param requestBody Request body, it can be null
     * @return A Future representing the result of the operation
     * @throws RuntimeException If an error occurs
     */
    public <T> CompletableFuture<T> request(Class<T> respClass, String method, String path, Object requestBody)
            throws RuntimeException {
        return doRequest(respClass, method, path, requestBody, null);
    }

    /**
     * Performs a HTTP request with headers
     * 
     * @param <T>         Response class type
     * @param respClass   Response class object, it can be null
     * @param method      HTTP method, e.g. get, post
     * @param path        Request path
     * @param requestBody Request body, it can be null
     * @param headers     Request headers, it can be null
     * @return A Future representing the result of the operation
     * @throws RuntimeException
     */
    public <T> CompletableFuture<T> request(Class<T> respClass, String method, String path, Object requestBody,
            HashMap<String, String> headers)
            throws RuntimeException {
        return doRequest(respClass, method, path, requestBody, headers);
    }

    private <T> CompletableFuture<T> doRequest(Class<T> respClass, String method, String path, Object requestBody,
            HashMap<String, String> headers)
            throws RuntimeException {
        Gson gson = new Gson();
        HashMap<String, Object> request = new HashMap<String, Object>();

        request.put("method", method);
        request.put("path", path);

        if (requestBody != null) {
            request.put("data", requestBody);
        }

        if (headers != null) {
            request.put("headers", headers);
        }

        String requestJson = gson.toJson(request);
        CompletableFuture<String> fut = AsyncCallback.executeTask((callback) -> {
            SdkNative.httpClientRequest(this.raw, requestJson, callback);
        });
        return fut.thenApply(respBody -> {
            if (respClass != null) {
                return gson.fromJson(respBody, respClass);
            } else {
                return null;
            }
        });
    }
}
