package com.longport;

import java.util.concurrent.CompletableFuture;

/**
 * @hidden
 */
public interface AsyncCallback {
    public void callback(Object err, Object obj);

    public static interface AsyncTask {
        void run(AsyncCallback callback);
    }

    @SuppressWarnings("unchecked")
    public static <T> CompletableFuture<T> executeTask(AsyncTask task) {
        CompletableFuture<T> fut = new CompletableFuture<T>();
        task.run((err, obj) -> {
            if (err == null) {
                fut.complete((T) obj);
            } else {
                fut.completeExceptionally((Throwable) err);
            }
        });
        return fut;
    }
}
