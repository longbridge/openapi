use std::{sync::Arc, time::Duration};

use futures_util::{Stream, StreamExt};
use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Deserialize, Serialize};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, agent::types::*};

/// The shared httpclient default (30s) is tuned for fast REST calls and is
/// too tight here: in blocking mode the server holds the connection silent
/// until the whole LLM turn is done, and that can legitimately take longer.
/// Only agent calls get this longer budget — every other domain keeps the
/// 30s default.
const AGENT_REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

struct InnerAgentContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerAgentContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("agent context dropped");
        });
    }
}

/// AI Agent conversation context.
///
/// Reference: <https://open.longbridge.com/en/docs/ai/chat/conversation>
#[derive(Clone)]
pub struct AgentContext(Arc<InnerAgentContext>);

#[derive(Debug, Deserialize)]
struct SseEnvelope {
    event: String,
    #[serde(default)]
    data: serde_json::Value,
    /// Only present on `plan_changed`, as a sibling of `data` rather than a
    /// field inside it — see [`PlanChangedPayload::tool_name`].
    #[serde(default)]
    tool_name: Option<String>,
}

/// Parse one raw SSE frame into a [`ConversationStreamEvent`], threading the
/// `chat_uid`/`message_id` captured from an earlier `chat_started` event (the
/// `workflow_finished` event doesn't repeat them) through `started`.
fn map_conversation_event(
    item: longbridge_httpcli::HttpClientResult<longbridge_httpcli::SseEvent>,
    started: &mut Option<(String, String)>,
) -> Result<ConversationStreamEvent> {
    let event = item?;
    let envelope: SseEnvelope = serde_json::from_str(&event.data)?;
    Ok(match envelope.event.as_str() {
        "chat_started" => {
            let payload: ChatStartedPayload = serde_json::from_value(envelope.data)?;
            *started = Some((payload.chat_uid.clone(), payload.message_id.clone()));
            ConversationStreamEvent::ChatStarted(payload)
        }
        "message" => ConversationStreamEvent::Message(serde_json::from_value(envelope.data)?),
        "workflow_started" => {
            ConversationStreamEvent::WorkflowStarted(serde_json::from_value(envelope.data)?)
        }
        "ping" => ConversationStreamEvent::Ping,
        "thinking_started" => {
            ConversationStreamEvent::ThinkingStarted(serde_json::from_value(envelope.data)?)
        }
        "thinking_finished" => {
            ConversationStreamEvent::ThinkingFinished(serde_json::from_value(envelope.data)?)
        }
        "node_tool_use_started" => {
            ConversationStreamEvent::NodeToolUseStarted(serde_json::from_value(envelope.data)?)
        }
        "node_tool_use_finished" => {
            ConversationStreamEvent::NodeToolUseFinished(serde_json::from_value(envelope.data)?)
        }
        "subagent_started" => {
            ConversationStreamEvent::SubagentStarted(serde_json::from_value(envelope.data)?)
        }
        "subagent_progress" => {
            ConversationStreamEvent::SubagentProgress(serde_json::from_value(envelope.data)?)
        }
        "subagent_finished" => {
            ConversationStreamEvent::SubagentFinished(serde_json::from_value(envelope.data)?)
        }
        "agent_tool_started" => {
            ConversationStreamEvent::AgentToolStarted(serde_json::from_value(envelope.data)?)
        }
        "agent_tool_progress" => {
            ConversationStreamEvent::AgentToolProgress(serde_json::from_value(envelope.data)?)
        }
        "agent_tool_finished" => {
            ConversationStreamEvent::AgentToolFinished(serde_json::from_value(envelope.data)?)
        }
        "human_interaction_required" => {
            let interrupt: Interrupt = serde_json::from_value(envelope.data)?;
            ConversationStreamEvent::HumanInteractionRequired(
                ConversationResponse::from_stream_interrupt(started.clone(), interrupt),
            )
        }
        "query_masked" => {
            ConversationStreamEvent::QueryMasked(serde_json::from_value(envelope.data)?)
        }
        "plan_changed" => {
            let mut payload: PlanChangedPayload = serde_json::from_value(envelope.data)?;
            payload.tool_name = envelope.tool_name.clone().unwrap_or_default();
            ConversationStreamEvent::PlanChanged(payload)
        }
        "context_compress_started" => {
            ConversationStreamEvent::ContextCompressStarted(serde_json::from_value(envelope.data)?)
        }
        "context_compress_finished" => {
            ConversationStreamEvent::ContextCompressFinished(serde_json::from_value(envelope.data)?)
        }
        "chat_finished" => {
            ConversationStreamEvent::ChatFinished(serde_json::from_value(envelope.data)?)
        }
        "chat_title_updated" => {
            ConversationStreamEvent::ChatTitleUpdated(serde_json::from_value(envelope.data)?)
        }
        "workflow_finished" => {
            let payload: WorkflowFinishedPayload = serde_json::from_value(envelope.data)?;
            ConversationStreamEvent::WorkflowFinished(ConversationResponse::from_stream_parts(
                started.clone(),
                payload,
            ))
        }
        _ => ConversationStreamEvent::Other {
            event: envelope.event,
            data: envelope.data,
        },
    })
}

impl AgentContext {
    /// Create an [`AgentContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("agent");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating agent context");
        });
        let ctx = Self(Arc::new(InnerAgentContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("agent context created");
        });
        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// List the Workspaces the current account belongs to.
    ///
    /// Path: `GET /v1/ai/workspaces`
    pub async fn workspaces(&self) -> Result<WorkspacesResponse> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/ai/workspaces")
            .response::<Json<WorkspacesResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// List the Agents in the specified Workspace.
    ///
    /// Path: `GET /v1/ai/workspaces/{id}/agents`
    pub async fn agents(
        &self,
        workspace_id: impl Into<String>,
        opts: impl Into<Option<GetAgentsOptions>>,
    ) -> Result<AgentsResponse> {
        let workspace_id = workspace_id.into();
        Ok(self
            .0
            .http_cli
            .request(
                Method::GET,
                format!("/v1/ai/workspaces/{workspace_id}/agents"),
            )
            .query_params(opts.into().unwrap_or_default())
            .response::<Json<AgentsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Start a conversation with the specified Agent, blocking until the run
    /// succeeds, is interrupted, or fails.
    ///
    /// Path: `POST /v1/ai/agents/{id}/conversations`
    pub async fn conversation(
        &self,
        agent_id: impl Into<String>,
        query: impl Into<String>,
        chat_uid: impl Into<Option<String>>,
    ) -> Result<ConversationResponse> {
        #[derive(Debug, Serialize)]
        struct Body {
            query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_uid: Option<String>,
        }

        let agent_id = agent_id.into();
        Ok(self
            .0
            .http_cli
            .request(
                Method::POST,
                format!("/v1/ai/agents/{agent_id}/conversations"),
            )
            .header("Accept", "application/json")
            .body(Json(Body {
                query: query.into(),
                chat_uid: chat_uid.into(),
            }))
            .timeout(AGENT_REQUEST_TIMEOUT)
            .response::<Json<ConversationResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Resume an interrupted conversation, blocking until the run succeeds, is
    /// interrupted again, or fails.
    ///
    /// Path: `POST
    /// /v1/ai/agents/{id}/conversations/{chat_uid}/messages/{message_id}/
    /// continue`
    pub async fn continue_conversation(
        &self,
        agent_id: impl Into<String>,
        chat_uid: impl Into<String>,
        message_id: impl Into<String>,
        answers: AnswersByToolCall,
    ) -> Result<ConversationResponse> {
        #[derive(Debug, Serialize)]
        struct Body {
            answers_by_tool_call: AnswersByToolCall,
        }

        let agent_id = agent_id.into();
        let chat_uid = chat_uid.into();
        let message_id = message_id.into();
        Ok(self
            .0
            .http_cli
            .request(
                Method::POST,
                format!(
                    "/v1/ai/agents/{agent_id}/conversations/{chat_uid}/messages/{message_id}/continue"
                ),
            )
            .header("Accept", "application/json")
            .body(Json(Body {
                answers_by_tool_call: answers,
            }))
            .timeout(AGENT_REQUEST_TIMEOUT)
            .response::<Json<ConversationResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Start a conversation with the specified Agent, returning a [`Stream`] of
    /// run-progress events over SSE. The run's outcome is carried by a
    /// [`ConversationStreamEvent::WorkflowFinished`] event (succeeded, failed,
    /// or stopped) or, if the Agent needs more input from you, a
    /// [`ConversationStreamEvent::HumanInteractionRequired`] event instead —
    /// an interrupted run never emits `WorkflowFinished`. Neither is
    /// necessarily the last item — the server may still emit a few more
    /// housekeeping events (e.g.
    /// [`ConversationStreamEvent::ChatTitleUpdated`]) before actually closing
    /// the connection, so keep draining the stream until it ends rather than
    /// stopping as soon as you see one.
    ///
    /// Path: `POST /v1/ai/agents/{id}/conversations` (`Accept:
    /// text/event-stream`)
    pub async fn conversation_streamed(
        &self,
        agent_id: impl Into<String>,
        query: impl Into<String>,
        chat_uid: impl Into<Option<String>>,
    ) -> Result<impl Stream<Item = Result<ConversationStreamEvent>> + Send + 'static> {
        #[derive(Debug, Serialize)]
        struct Body {
            query: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_uid: Option<String>,
        }

        let agent_id = agent_id.into();
        let raw = self
            .0
            .http_cli
            .request(
                Method::POST,
                format!("/v1/ai/agents/{agent_id}/conversations"),
            )
            .body(Json(Body {
                query: query.into(),
                chat_uid: chat_uid.into(),
            }))
            .timeout(AGENT_REQUEST_TIMEOUT)
            .send_events()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;

        let mut started: Option<(String, String)> = None;
        Ok(raw.map(move |item| map_conversation_event(item, &mut started)))
    }

    /// Resume an interrupted conversation, returning a [`Stream`] of
    /// run-progress events over SSE.
    ///
    /// Path: `POST
    /// /v1/ai/agents/{id}/conversations/{chat_uid}/messages/{message_id}/
    /// continue` (`Accept: text/event-stream`)
    pub async fn continue_conversation_streamed(
        &self,
        agent_id: impl Into<String>,
        chat_uid: impl Into<String>,
        message_id: impl Into<String>,
        answers: AnswersByToolCall,
    ) -> Result<impl Stream<Item = Result<ConversationStreamEvent>> + Send + 'static> {
        #[derive(Debug, Serialize)]
        struct Body {
            answers_by_tool_call: AnswersByToolCall,
        }

        let agent_id = agent_id.into();
        let chat_uid = chat_uid.into();
        let message_id = message_id.into();
        // We already know chat_uid/message_id from the caller (unlike a brand-new
        // conversation) — seed `started` so the final ConversationResponse carries
        // them even if the server doesn't re-emit a `chat_started` event here.
        let mut started = Some((chat_uid.clone(), message_id.clone()));
        let raw = self
            .0
            .http_cli
            .request(
                Method::POST,
                format!(
                    "/v1/ai/agents/{agent_id}/conversations/{chat_uid}/messages/{message_id}/continue"
                ),
            )
            .body(Json(Body {
                answers_by_tool_call: answers,
            }))
            .timeout(AGENT_REQUEST_TIMEOUT)
            .send_events()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;

        Ok(raw.map(move |item| map_conversation_event(item, &mut started)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The `data:` payloads of the three example SSE frames from
    // https://open.longbridge.com/en/docs/ai/chat/conversation
    const CHAT_STARTED: &str = r#"{"event":"chat_started","workflow_run_id":"wr_1","data":{"chat_uid":"ct_9f2c1a5b","message_id":42}}"#;
    const MESSAGE: &str = r#"{"event":"message","workflow_run_id":"wr_1","data":{"text":"Tesla"}}"#;
    const WORKFLOW_FINISHED: &str = r#"{"event":"workflow_finished","workflow_run_id":"wr_1","data":{"status":"succeeded","elapsed_time":3.21,"outputs":{"answer":"Tesla (TSLA.US) recently..."}}}"#;

    // The four event types below aren't in the docs — captured verbatim from
    // real traffic during manual live testing (see conversation history).
    const WORKFLOW_STARTED: &str = r#"{"event":"workflow_started","workflow_run_id":"wr_1","data":{"hit_cache":false,"inputs":{"chat_id":834552,"chat_uid":"ct_9f2c1a5b","message_id":42,"query":"How has Tesla stock performed recently?"},"started_at":1784545150,"workflow_id":176476}}"#;
    const PING: &str = r#"{"event":"ping","workflow_run_id":"wr_1","data":null}"#;
    const CHAT_FINISHED: &str = r#"{"event":"chat_finished","workflow_run_id":"wr_1","data":{"chat_id":834552,"chat_uid":"ct_9f2c1a5b","error":"","error_message":"","message_id":42}}"#;
    const CHAT_TITLE_UPDATED: &str = r#"{"event":"chat_title_updated","workflow_run_id":"wr_1","data":{"chat_id":834552,"chat_uid":"ct_9f2c1a5b","source":"ai_generated","title":"Tesla stock performance","updated_at":1784546957}}"#;

    fn sse(data: &str) -> longbridge_httpcli::HttpClientResult<longbridge_httpcli::SseEvent> {
        Ok(longbridge_httpcli::SseEvent {
            event: "message".to_string(),
            data: data.to_string(),
            id: String::new(),
            retry: None,
        })
    }

    #[test]
    fn map_conversation_event_full_sequence() {
        let mut started = None;

        match map_conversation_event(sse(CHAT_STARTED), &mut started).unwrap() {
            ConversationStreamEvent::ChatStarted(payload) => {
                assert_eq!(payload.chat_uid, "ct_9f2c1a5b");
                assert_eq!(payload.message_id, "42");
            }
            other => panic!("unexpected event: {other:?}"),
        }
        assert_eq!(started, Some(("ct_9f2c1a5b".to_string(), "42".to_string())));

        // The real event stream is richer than the docs' three-event example
        // — this exercises the fuller, real-world sequence, including
        // `chat_title_updated` arriving *after* `workflow_finished` (observed
        // live; see the "drain to the stream's natural end" fix).
        match map_conversation_event(sse(WORKFLOW_STARTED), &mut started).unwrap() {
            ConversationStreamEvent::WorkflowStarted(payload) => {
                assert!(!payload.hit_cache);
                assert_eq!(payload.inputs.chat_uid, "ct_9f2c1a5b");
                assert_eq!(payload.inputs.message_id, "42");
                assert_eq!(payload.workflow_id, 176476);
            }
            other => panic!("unexpected event: {other:?}"),
        }

        match map_conversation_event(sse(MESSAGE), &mut started).unwrap() {
            ConversationStreamEvent::Message(payload) => assert_eq!(payload.text, "Tesla"),
            other => panic!("unexpected event: {other:?}"),
        }

        match map_conversation_event(sse(PING), &mut started).unwrap() {
            ConversationStreamEvent::Ping => {}
            other => panic!("unexpected event: {other:?}"),
        }

        match map_conversation_event(sse(CHAT_FINISHED), &mut started).unwrap() {
            ConversationStreamEvent::ChatFinished(payload) => {
                assert_eq!(payload.chat_uid, "ct_9f2c1a5b");
                assert_eq!(payload.message_id, "42");
                assert_eq!(payload.error, "");
                assert_eq!(payload.error_message, "");
            }
            other => panic!("unexpected event: {other:?}"),
        }

        match map_conversation_event(sse(WORKFLOW_FINISHED), &mut started).unwrap() {
            ConversationStreamEvent::WorkflowFinished(resp) => {
                assert_eq!(resp.chat_uid, "ct_9f2c1a5b");
                assert_eq!(resp.message_id, "42");
                assert_eq!(resp.status, ConversationStatus::Succeeded);
                assert_eq!(resp.answer, "Tesla (TSLA.US) recently...");
            }
            other => panic!("unexpected event: {other:?}"),
        }

        // Arrives *after* workflow_finished in this (real, observed) ordering.
        match map_conversation_event(sse(CHAT_TITLE_UPDATED), &mut started).unwrap() {
            ConversationStreamEvent::ChatTitleUpdated(payload) => {
                assert_eq!(payload.chat_uid, "ct_9f2c1a5b");
                assert_eq!(payload.source, "ai_generated");
                assert_eq!(payload.title, "Tesla stock performance");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn map_conversation_event_unknown_type_falls_back_to_other() {
        let mut started = None;
        let json = r#"{"event":"some_future_event","data":{"foo":"bar"}}"#;
        match map_conversation_event(sse(json), &mut started).unwrap() {
            ConversationStreamEvent::Other { event, data } => {
                assert_eq!(event, "some_future_event");
                assert_eq!(data["foo"], "bar");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    // https://github.com/longbridge/developers/pull/1176 — an interrupted
    // run's stream never emits `workflow_finished`; `human_interaction_required`
    // is the terminal event instead.
    const HUMAN_INTERACTION_REQUIRED: &str = r#"{"event":"human_interaction_required","workflow_run_id":"wr_1","data":{"node_id":"n_ask_human","tool_call_id":"call_abc123","questions":[{"question":"Which time range would you like to check?","options":[{"description":"Past week"},{"description":"Past month"}],"multi_select":false}],"message_id":43,"chat_id":1001}}"#;

    #[test]
    fn map_conversation_event_interrupted_sequence_has_no_workflow_finished() {
        let mut started = None;
        map_conversation_event(sse(CHAT_STARTED), &mut started).unwrap();
        map_conversation_event(sse(WORKFLOW_STARTED), &mut started).unwrap();

        match map_conversation_event(sse(HUMAN_INTERACTION_REQUIRED), &mut started).unwrap() {
            ConversationStreamEvent::HumanInteractionRequired(resp) => {
                assert_eq!(resp.chat_uid, "ct_9f2c1a5b");
                assert_eq!(resp.message_id, "42");
                assert_eq!(resp.status, ConversationStatus::Interrupted);
                let interrupt = resp.interrupt.expect("interrupt");
                assert_eq!(interrupt.node_id, "n_ask_human");
                assert_eq!(interrupt.tool_call_id, "call_abc123");
            }
            other => panic!("unexpected event: {other:?}"),
        }

        // The stream still ends with `chat_finished`, just never emits
        // `workflow_finished`.
        match map_conversation_event(sse(CHAT_FINISHED), &mut started).unwrap() {
            ConversationStreamEvent::ChatFinished(_) => {}
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn map_conversation_event_plan_changed_picks_up_sibling_tool_name() {
        let mut started = None;
        // `tool_name` sits outside `data`, as a sibling of `event`/`data` in
        // the raw envelope.
        let json = r#"{"event":"plan_changed","workflow_run_id":"wr_1","tool_name":"planner","data":{"node_id":"n_plan","started_at":1752048000}}"#;
        match map_conversation_event(sse(json), &mut started).unwrap() {
            ConversationStreamEvent::PlanChanged(payload) => {
                assert_eq!(payload.node_id, "n_plan");
                assert_eq!(payload.tool_name, "planner");
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }
}
