package com.longbridge.agent;

import java.util.Collections;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Flow;

import com.google.gson.Gson;
import com.longbridge.AsyncCallback;
import com.longbridge.Config;
import com.longbridge.OpenApiException;
import com.longbridge.SdkNative;

/**
 * AI Agent conversation context.
 * <p>
 * Reference: <a href="https://open.longbridge.com/en/docs/ai/chat/conversation">
 * https://open.longbridge.com/en/docs/ai/chat/conversation</a>
 */
public class AgentContext implements AutoCloseable {
    private long raw;

    /**
     * Create an AgentContext object
     *
     * @param config Config object
     * @return A AgentContext object
     */
    public static AgentContext create(Config config) {
        AgentContext ctx = new AgentContext();
        ctx.raw = SdkNative.newAgentContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeAgentContext(raw);
    }

    /**
     * List the Workspaces the current account belongs to.
     *
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<WorkspacesResponse> workspaces() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.agentContextWorkspaces(this.raw, callback);
        });
    }

    /**
     * List the Agents in the specified Workspace.
     *
     * @param workspaceId Workspace ID
     * @param opts        Options for this request, may be {@code null}
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<AgentsResponse> agents(String workspaceId, GetAgentsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.agentContextAgents(this.raw, workspaceId, opts, callback);
        });
    }

    /**
     * Start a conversation with the specified Agent, blocking until the run
     * succeeds, is interrupted, or fails.
     *
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.agent.*;
     *
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); AgentContext ctx = AgentContext.create(config)) {
     *             WorkspacesResponse workspaces = ctx.workspaces().get();
     *             AgentsResponse agents = ctx.agents(workspaces.getWorkspaces()[0].getId(), null).get();
     *             ConversationResponse resp = ctx.conversation(agents.getAgents()[0].getUid(),
     *                     "How has Tesla stock performed recently?", null).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     *
     * @param agentId Agent UID
     * @param query   User query
     * @param chatUid Conversation identifier to continue an existing chat, or
     *                {@code null} to start a new one
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ConversationResponse> conversation(String agentId, String query, String chatUid)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.agentContextConversation(this.raw, agentId, query, chatUid, callback);
        });
    }

    /**
     * Resume an interrupted conversation, blocking until the run succeeds, is
     * interrupted again, or fails.
     *
     * @param agentId          Agent UID
     * @param chatUid          Conversation identifier
     * @param messageId        ID of the paused message (see
     *                         {@link Interrupt#getMessageId})
     * @param answersByToolCall Answers keyed by {@code toolCallId}, each value
     *                         being a map of question text to answer; may be
     *                         {@code null} if there is nothing to answer
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ConversationResponse> continueConversation(String agentId, String chatUid,
            String messageId, Map<String, Map<String, String>> answersByToolCall) throws OpenApiException {
        String answersJson = toAnswersJson(answersByToolCall);
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.agentContextContinueConversation(this.raw, agentId, chatUid, messageId, answersJson, callback);
        });
    }

    /**
     * Start a conversation with the specified Agent, returning a
     * {@link Flow.Publisher} of run-progress events over SSE. The run's
     * outcome is carried by a {@link WorkflowFinishedEvent} (succeeded,
     * failed, or stopped) or, if the Agent needs more input from you, a
     * {@link HumanInteractionRequiredEvent} instead (unless the stream itself
     * errors first, delivered via {@code Flow.Subscriber#onError}) — an
     * interrupted run never emits a {@link WorkflowFinishedEvent}. Neither is
     * necessarily the last event delivered — the server may still emit a few
     * more housekeeping events (e.g. a {@link ChatTitleUpdatedEvent}) before
     * actually closing the connection, so keep consuming until
     * {@code onComplete} rather than stopping as soon as you see one.
     * <p>
     * This method itself performs no I/O — it returns a cold
     * {@link Flow.Publisher} immediately; the HTTP/SSE connection is only
     * established once a subscriber calls {@code subscribe}, matching
     * Reactive Streams' lazy-publisher convention. The returned publisher
     * carries real backpressure: no more events are pulled off the stream
     * than have been requested via {@link Flow.Subscription#request}.
     *
     * <pre>
     * {@code
     * Flow.Publisher<ConversationStreamEvent> publisher =
     *     ctx.conversationStream(agentId, "How has Tesla stock performed recently?", null);
     * publisher.subscribe(new Flow.Subscriber<ConversationStreamEvent>() {
     *     public void onSubscribe(Flow.Subscription subscription) {
     *         subscription.request(Long.MAX_VALUE); // unbounded demand
     *     }
     *     public void onNext(ConversationStreamEvent event) {
     *         System.out.println(event);
     *     }
     *     public void onError(Throwable err) {
     *         System.out.println("failed: " + err.getMessage());
     *     }
     *     public void onComplete() {
     *         System.out.println("done");
     *     }
     * });
     * }
     * </pre>
     *
     * @param agentId Agent UID
     * @param query   User query
     * @param chatUid Conversation identifier to continue an existing chat, or
     *                {@code null} to start a new one
     * @return A cold {@link Flow.Publisher} of conversation stream events
     */
    public Flow.Publisher<ConversationStreamEvent> conversationStream(String agentId, String query, String chatUid) {
        return new ConversationStreamPublisher(this.raw, agentId, query, chatUid);
    }

    /**
     * Resume an interrupted conversation, returning a {@link Flow.Publisher}
     * of run-progress events over SSE. See {@link #conversationStream} for
     * the lazy-publisher/backpressure semantics.
     *
     * @param agentId          Agent UID
     * @param chatUid          Conversation identifier
     * @param messageId        ID of the paused message (see
     *                         {@link Interrupt#getMessageId})
     * @param answersByToolCall Answers keyed by {@code toolCallId}, each value
     *                         being a map of question text to answer; may be
     *                         {@code null} if there is nothing to answer
     * @return A cold {@link Flow.Publisher} of conversation stream events
     */
    public Flow.Publisher<ConversationStreamEvent> continueConversationStream(String agentId, String chatUid,
            String messageId, Map<String, Map<String, String>> answersByToolCall) {
        String answersJson = toAnswersJson(answersByToolCall);
        return new ConversationStreamPublisher(this.raw, agentId, chatUid, messageId, answersJson);
    }

    private static String toAnswersJson(Map<String, Map<String, String>> answersByToolCall) {
        Map<String, Map<String, String>> answers = answersByToolCall != null ? answersByToolCall
                : Collections.emptyMap();
        return new Gson().toJson(answers);
    }
}
