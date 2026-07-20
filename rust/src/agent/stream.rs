use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use futures_util::{Stream, StreamExt};
use tokio::{sync::Notify, task::AbortHandle};

use crate::{
    Error, Result,
    agent::types::{ConversationResponse, ConversationStreamEvent},
};

/// Drive a conversation event stream to completion, invoking `on_event` for
/// every event, and returning the final [`ConversationResponse`] once a
/// [`ConversationStreamEvent::WorkflowFinished`] event is observed (or an error
/// if the stream ends before that happens).
///
/// Used by binding layers that are call-scoped-callback shaped (C, C++,
/// Node.js) — every other binding either pulls synchronously
/// ([`conversation_stream_iter`]) or drives with real backpressure
/// ([`ConversationStreamSubscription`]).
pub async fn drive_conversation_stream<S, F>(
    mut stream: S,
    mut on_event: F,
) -> Result<ConversationResponse>
where
    S: Stream<Item = Result<ConversationStreamEvent>> + Send + Unpin,
    F: FnMut(ConversationStreamEvent) + Send,
{
    let mut final_response = None;
    while let Some(event) = stream.next().await {
        let event = event?;
        if let ConversationStreamEvent::WorkflowFinished(ref resp) = event {
            final_response = Some(resp.clone());
        }
        on_event(event);
    }
    final_response.ok_or(Error::ConversationStreamEnded)
}

/// A blocking [`Iterator`] over conversation stream events, backed by a
/// background task on the shared runtime ([`crate::runtime_handle`]). Useful
/// for sync/FFI bindings that need to pull events one at a time from a plain OS
/// thread instead of polling a [`Stream`] directly.
pub struct ConversationStreamIter(std::sync::mpsc::Receiver<Result<ConversationStreamEvent>>);

impl Iterator for ConversationStreamIter {
    type Item = Result<ConversationStreamEvent>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.recv().ok()
    }
}

/// Adapt a conversation event [`Stream`] into a blocking
/// [`ConversationStreamIter`].
pub fn conversation_stream_iter(
    stream: impl Stream<Item = Result<ConversationStreamEvent>> + Send + 'static,
) -> ConversationStreamIter {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut stream = Box::pin(stream);
    crate::runtime_handle().spawn(async move {
        while let Some(item) = stream.next().await {
            if tx.send(item).is_err() {
                break; // receiver dropped, caller stopped iterating early
            }
        }
    });
    ConversationStreamIter(rx)
}

/// Bridges a conversation event [`Stream`] to a Reactive-Streams-style consumer
/// with real backpressure, matching `java.util.concurrent.Flow.Subscription`'s
/// `request(n)`/`cancel()` contract. Only Java's `Flow.Publisher` exposure
/// needs this — every other binding either pulls synchronously
/// ([`conversation_stream_iter`]) or has no flow control at all
/// ([`drive_conversation_stream`]).
pub struct ConversationStreamSubscription {
    demand: Arc<(AtomicU64, Notify)>,
    abort: AbortHandle,
}

impl ConversationStreamSubscription {
    /// Spawn a background task that waits for demand, pulls one item at a time
    /// from `stream` once demand is available, and dispatches
    /// `on_next`/`on_error`/`on_complete` (each of these is expected to call
    /// back into the JVM via a JNI `Subscriber` reference).
    ///
    /// Drains all the way to the stream's natural end rather than stopping as
    /// soon as a [`ConversationStreamEvent::WorkflowFinished`] is seen —
    /// against the real API, the server sometimes emits a few more
    /// housekeeping events (e.g. a `chat_title_updated`-shaped
    /// [`ConversationStreamEvent::Other`]) after `workflow_finished` and
    /// before actually closing the connection, so stopping early would
    /// silently drop them and abandon the connection while the server still
    /// had something to say.
    pub fn spawn<S, F1, F2, F3>(stream: S, on_next: F1, on_error: F2, on_complete: F3) -> Self
    where
        S: Stream<Item = Result<ConversationStreamEvent>> + Send + 'static,
        F1: Fn(ConversationStreamEvent) + Send + Sync + 'static,
        F2: FnOnce(Error) + Send + 'static,
        F3: FnOnce() + Send + 'static,
    {
        let demand = Arc::new((AtomicU64::new(0), Notify::new()));
        let demand2 = demand.clone();
        let handle = crate::runtime_handle().spawn(async move {
            let mut stream = Box::pin(stream);
            loop {
                // wait until `request(n)` has added at least one credit
                while demand2.0.load(Ordering::Acquire) == 0 {
                    demand2.1.notified().await;
                }
                match stream.next().await {
                    Some(Ok(event)) => {
                        demand2.0.fetch_sub(1, Ordering::AcqRel);
                        on_next(event);
                    }
                    Some(Err(err)) => {
                        on_error(err);
                        break;
                    }
                    None => {
                        on_complete();
                        break;
                    }
                }
            }
        });
        Self {
            demand,
            abort: handle.abort_handle(),
        }
    }

    /// Called from `Flow.Subscription.request(n)` (any JVM thread).
    pub fn request(&self, n: u64) {
        self.demand.0.fetch_add(n, Ordering::AcqRel);
        self.demand.1.notify_one();
    }

    /// Called from `Flow.Subscription.cancel()`.
    pub fn cancel(&self) {
        self.abort.abort();
    }
}
