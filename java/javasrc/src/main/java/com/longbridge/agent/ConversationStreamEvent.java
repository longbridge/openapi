package com.longbridge.agent;

/**
 * One event observed while streaming {@link AgentContext#conversationStream}
 * or {@link AgentContext#continueConversationStream}.
 * <p>
 * This is a sealed-style class hierarchy mirroring the Rust
 * {@code ConversationStreamEvent} enum: each concrete subclass below carries
 * exactly one variant's payload. Use {@code instanceof} (or a pattern-matching
 * {@code switch} on Java 21+) to dispatch on the concrete type:
 *
 * <pre>{@code
 * publisher.subscribe(new Flow.Subscriber<ConversationStreamEvent>() {
 *     public void onNext(ConversationStreamEvent event) {
 *         if (event instanceof MessageEvent message) {
 *             System.out.print(message.getText());
 *         } else if (event instanceof WorkflowFinishedEvent finished) {
 *             System.out.println(finished.getResponse());
 *         }
 *     }
 *     // ...
 * });
 * }</pre>
 *
 * @see ChatStartedEvent
 * @see MessageEvent
 * @see WorkflowFinishedEvent
 * @see OtherEvent
 */
public abstract class ConversationStreamEvent {
    ConversationStreamEvent() {
    }
}
