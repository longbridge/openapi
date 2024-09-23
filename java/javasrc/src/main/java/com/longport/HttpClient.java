package com.longport;

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

    public HttpClient(String httpUrl, String appKey, String appSecret, String accessToken) {
        this.raw = SdkNative.newHttpClient(httpUrl, appKey, appSecret, accessToken);
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeHttpClient(this.raw);
    }

    /**
     * Create a new `HttpClient` from the given environment variables
     * <p>
     * It first gets the environment variables from the .env file in the current
     * directory.
     * 
     * # Variables
     * 
     * - `LONGPORT_HTTP_URL` - HTTP endpoint url
     * - `LONGPORT_APP_KEY` - App key
     * - `LONGPORT_APP_SECRET` - App secret
     * - `LONGPORT_ACCESS_TOKEN` - Access token
     * 
     * @return Config object
     * @throws OpenApiException If an error occurs
     */
    public static HttpClient fromEnv() throws OpenApiException {
        return new HttpClient(SdkNative.newHttpClientFromEnv());
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
