use std::{collections::HashMap, sync::Arc};

use napi::{Result, threadsafe_function::ThreadsafeFunctionCallMode};

use crate::{agent::types::*, config::Config, error::ErrorNewType, utils::JsCallback};

/// AI Agent conversation context.
///
/// Reference: <https://open.longbridge.com/en/docs/ai/chat/conversation>
#[napi_derive::napi]
#[derive(Clone)]
pub struct AgentContext {
    ctx: longbridge::AgentContext,
}

#[napi_derive::napi]
impl AgentContext {
    /// Create a new AgentContext.
    #[napi]
    pub fn new(config: &Config) -> AgentContext {
        Self {
            ctx: longbridge::AgentContext::new(Arc::new(config.0.clone())),
        }
    }

    /// List the Workspaces the current account belongs to.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, AgentContext } = require('longbridge');
    ///
    /// const ctx = AgentContext.new(config);
    /// const resp = await ctx.workspaces();
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn workspaces(&self) -> Result<WorkspacesResponse> {
        Ok(self.ctx.workspaces().await.map_err(ErrorNewType)?.into())
    }

    /// List the Agents in the specified Workspace.
    ///
    /// `page`/`limit` control pagination; `name` fuzzy-searches by Agent name.
    /// All three are optional.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, AgentContext } = require('longbridge');
    ///
    /// const ctx = AgentContext.new(config);
    /// const resp = await ctx.agents(workspaceId);
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn agents(
        &self,
        workspace_id: String,
        page: Option<i32>,
        limit: Option<i32>,
        name: Option<String>,
    ) -> Result<AgentsResponse> {
        let mut opts = longbridge::agent::GetAgentsOptions::new();
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        if let Some(name) = name {
            opts = opts.name(name);
        }
        Ok(self
            .ctx
            .agents(workspace_id, opts)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Start a conversation with the specified Agent, blocking until the run
    /// succeeds, is interrupted, or fails.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, AgentContext } = require('longbridge');
    ///
    /// const ctx = AgentContext.new(config);
    /// const resp = await ctx.conversation(agentId, "How has Tesla stock performed recently?");
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn conversation(
        &self,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
    ) -> Result<ConversationResponse> {
        Ok(self
            .ctx
            .conversation(agent_id, query, chat_uid)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Resume an interrupted conversation, blocking until the run succeeds, is
    /// interrupted again, or fails.
    ///
    /// `answersByToolCall` is keyed by `toolCallId` (see `Interrupt`), each
    /// value being a map of question text to answer.
    #[napi]
    pub async fn continue_conversation(
        &self,
        agent_id: String,
        chat_uid: String,
        message_id: String,
        answers_by_tool_call: HashMap<String, HashMap<String, String>>,
    ) -> Result<ConversationResponse> {
        Ok(self
            .ctx
            .continue_conversation(agent_id, chat_uid, message_id, answers_by_tool_call)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Start a conversation with the specified Agent, invoking `callback` for
    /// every progress event observed over SSE, and resolving to the final
    /// `ConversationResponse` once the run finishes (this is the same shape
    /// `conversation` returns).
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { Config, AgentContext } = require('longbridge');
    ///
    /// const ctx = AgentContext.new(config);
    /// const resp = await ctx.conversationStreamed(
    ///   agentId,
    ///   "How has Tesla stock performed recently?",
    ///   undefined,
    ///   (err, event) => console.log(event),
    /// );
    /// console.log(resp);
    /// ```
    // Design note: unlike `QuoteContext::set_on_quote` (a plain, non-async
    // `fn` that takes a `Function<...>` and builds the threadsafe function
    // itself), `callback` here is declared as `JsCallback<ConversationStreamEvent>`
    // (i.e. `ThreadsafeFunction<...>`) directly. napi-rs converts the JS
    // function into a `ThreadsafeFunction` at the FFI boundary before this
    // `async fn`'s body ever runs, so no `!Send` `Function` value is ever
    // held across an `.await` point. Accepting a bare `Function` here and
    // building the threadsafe function manually inside the body — mirroring
    // `set_on_quote` — makes the generated future `!Send` (`Function` wraps
    // raw `napi_env`/`napi_value` pointers), and `execute_tokio_future`
    // requires the future backing every `#[napi] async fn` to be `Send`.
    // `set_on_quote` itself avoids this only because it is not async.
    #[napi(
        ts_args_type = "agentId: string, query: string, chatUid: string | undefined | null, callback: (err: null | Error, event: ConversationStreamEvent) => void"
    )]
    pub async fn conversation_streamed(
        &self,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
        callback: JsCallback<ConversationStreamEvent>,
    ) -> Result<ConversationResponse> {
        let stream = self
            .ctx
            .conversation_streamed(agent_id, query, chat_uid)
            .await
            .map_err(ErrorNewType)?;
        Ok(
            longbridge::agent::drive_conversation_stream(stream, move |ev| {
                callback.call(Ok(ev.into()), ThreadsafeFunctionCallMode::Blocking);
            })
            .await
            .map_err(ErrorNewType)?
            .into(),
        )
    }

    /// Resume an interrupted conversation, invoking `callback` for every
    /// progress event observed over SSE, and resolving to the final
    /// `ConversationResponse` once the run finishes.
    ///
    /// `answersByToolCall` is keyed by `toolCallId` (see `Interrupt`), each
    /// value being a map of question text to answer.
    #[napi(
        ts_args_type = "agentId: string, chatUid: string, messageId: string, answersByToolCall: Record<string, Record<string, string>>, callback: (err: null | Error, event: ConversationStreamEvent) => void"
    )]
    pub async fn continue_conversation_streamed(
        &self,
        agent_id: String,
        chat_uid: String,
        message_id: String,
        answers_by_tool_call: HashMap<String, HashMap<String, String>>,
        callback: JsCallback<ConversationStreamEvent>,
    ) -> Result<ConversationResponse> {
        let stream = self
            .ctx
            .continue_conversation_streamed(agent_id, chat_uid, message_id, answers_by_tool_call)
            .await
            .map_err(ErrorNewType)?;
        Ok(
            longbridge::agent::drive_conversation_stream(stream, move |ev| {
                callback.call(Ok(ev.into()), ThreadsafeFunctionCallMode::Blocking);
            })
            .await
            .map_err(ErrorNewType)?
            .into(),
        )
    }
}
