package com.longbridge.agent;

import java.lang.ref.Cleaner;
import java.util.concurrent.Flow;

import com.longbridge.SdkNative;

/**
 * A {@link Flow.Subscription} for a conversation event stream.
 * <p>
 * This wraps a native handle backed by real demand/credit bookkeeping: no
 * more events are pulled off the underlying SSE stream than have been
 * {@link #request(long) requested}. Call {@link #request(long)} from
 * {@link Flow.Subscriber#onSubscribe} (and again whenever more events are
 * wanted), and {@link #cancel()} to stop the stream early.
 */
public class ConversationStreamSubscription implements Flow.Subscription {
    private static final Cleaner CLEANER = Cleaner.create();

    private final long raw;
    private final Cleaner.Cleanable cleanable;

    /**
     * @hidden
     */
    ConversationStreamSubscription(long raw) {
        this.raw = raw;
        // The cleaning action must not capture `this` (directly or via an
        // instance field reference), only the primitive `raw` handle — this
        // is the standard java.lang.ref.Cleaner idiom, since capturing `this`
        // would keep the object reachable forever and the action would never
        // run.
        this.cleanable = CLEANER.register(this, () -> SdkNative.freeConversationStreamSubscription(raw));
    }

    @Override
    public void request(long n) {
        SdkNative.conversationStreamSubscriptionRequest(raw, n);
    }

    @Override
    public void cancel() {
        SdkNative.conversationStreamSubscriptionCancel(raw);
    }
}
