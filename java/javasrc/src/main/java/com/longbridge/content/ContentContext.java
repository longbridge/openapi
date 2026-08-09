package com.longbridge.content;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Content context
 */
public class ContentContext implements AutoCloseable {
    private long raw;

    private long raw() {
        long r = this.raw;
        if (r == 0) {
            throw new IllegalStateException(
                    getClass().getSimpleName() + " has already been closed");
        }
        return r;
    }

    /**
     * Create a ContentContext object
     *
     * @param config Config object
     * @return A ContentContext object
     */
    public static ContentContext create(Config config) {
        ContentContext ctx = new ContentContext();
        ctx.raw = SdkNative.newContentContext(config.getRaw());
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeContentContext(h);
        }
    }

    /**
     * Get topics created by the current authenticated user
     *
     * @param opts Query options (page, size, topicType); may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<OwnedTopic[]> getMyTopics(MyTopicsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextMyTopics(raw(), opts, callback);
        });
    }

    /**
     * Create a new topic
     *
     * @param opts Create topic options
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<String> createTopic(CreateTopicOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextCreateTopic(raw(), opts, callback);
        });
    }

    /**
     * Get discussion topics list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<TopicItem[]> getTopics(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextTopics(raw(), symbol, callback);
        });
    }

    /**
     * Get news list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<NewsItem[]> getNews(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.contentContextNews(raw(), symbol, callback);
        });
    }
}
