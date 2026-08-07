package com.longbridge.agent;

import java.util.concurrent.Flow;

import com.longbridge.SdkNative;

/**
 * A cold {@link Flow.Publisher} of {@link ConversationStreamEvent}s.
 * <p>
 * Constructing this object (via {@link AgentContext#conversationStream} /
 * {@link AgentContext#continueConversationStream}) does not perform any I/O —
 * per Reactive Streams convention, the underlying HTTP/SSE connection is only
 * established once {@link #subscribe} is called, and a fresh, independent
 * connection is started for every subscriber.
 */
public class ConversationStreamPublisher implements Flow.Publisher<ConversationStreamEvent> {
    private final long ctx;
    private final String agentId;
    private final String query;
    private final String chatUid;
    private final String parentMessageId;
    private final String messageId;
    private final String answersByToolCallJson;

    private ConversationStreamPublisher(long ctx, String agentId, String query, String chatUid,
            String parentMessageId, String messageId, String answersByToolCallJson) {
        this.ctx = ctx;
        this.agentId = agentId;
        this.query = query;
        this.chatUid = chatUid;
        this.parentMessageId = parentMessageId;
        this.messageId = messageId;
        this.answersByToolCallJson = answersByToolCallJson;
    }

    /**
     * @hidden Constructs the "new conversation" variant, backing
     *         {@link AgentContext#conversationStream}.
     */
    static ConversationStreamPublisher forNewConversation(long ctx, String agentId, String query, String chatUid,
            String parentMessageId) {
        return new ConversationStreamPublisher(ctx, agentId, query, chatUid, parentMessageId, null, null);
    }

    /**
     * @hidden Constructs the "continue conversation" variant, backing
     *         {@link AgentContext#continueConversationStream}.
     */
    static ConversationStreamPublisher forContinueConversation(long ctx, String agentId, String chatUid,
            String messageId, String answersByToolCallJson) {
        return new ConversationStreamPublisher(ctx, agentId, null, chatUid, null, messageId, answersByToolCallJson);
    }

    @Override
    public void subscribe(Flow.Subscriber<? super ConversationStreamEvent> subscriber) {
        if (subscriber == null) {
            throw new NullPointerException("subscriber");
        }
        if (messageId == null) {
            SdkNative.agentContextConversationStreamSubscribe(ctx, agentId, query, chatUid, parentMessageId,
                    subscriber);
        } else {
            SdkNative.agentContextContinueConversationStreamSubscribe(ctx, agentId, chatUid, messageId,
                    answersByToolCallJson, subscriber);
        }
    }
}
