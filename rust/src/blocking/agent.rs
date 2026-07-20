use std::{pin::Pin, sync::Arc};

use futures_util::Stream;
use tokio::sync::mpsc;

use crate::{
    Config, Result,
    agent::{self, AgentContext, types::*},
    blocking::runtime::BlockingRuntime,
};

/// Blocking AI Agent conversation context.
pub struct AgentContextSync {
    rt: BlockingRuntime<AgentContext>,
}

impl AgentContextSync {
    /// Create an [`AgentContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = AgentContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// List the Workspaces the current account belongs to.
    pub fn workspaces(&self) -> Result<WorkspacesResponse> {
        self.rt
            .call(move |ctx| async move { ctx.workspaces().await })
    }

    /// List the Agents in the specified Workspace.
    pub fn agents(
        &self,
        workspace_id: impl Into<String> + Send + 'static,
        opts: impl Into<Option<GetAgentsOptions>> + Send + 'static,
    ) -> Result<AgentsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.agents(workspace_id, opts).await })
    }

    /// Start a conversation with the specified Agent, blocking until the run
    /// succeeds, is interrupted, or fails.
    pub fn conversation(
        &self,
        agent_id: impl Into<String> + Send + 'static,
        query: impl Into<String> + Send + 'static,
        chat_uid: Option<String>,
    ) -> Result<ConversationResponse> {
        self.rt
            .call(move |ctx| async move { ctx.conversation(agent_id, query, chat_uid).await })
    }

    /// Resume an interrupted conversation, blocking until the run succeeds, is
    /// interrupted again, or fails.
    pub fn continue_conversation(
        &self,
        agent_id: impl Into<String> + Send + 'static,
        chat_uid: impl Into<String> + Send + 'static,
        message_id: impl Into<String> + Send + 'static,
        answers: AnswersByToolCall,
    ) -> Result<ConversationResponse> {
        self.rt.call(move |ctx| async move {
            ctx.continue_conversation(agent_id, chat_uid, message_id, answers)
                .await
        })
    }

    /// Start a conversation with the specified Agent, returning a blocking
    /// [`agent::ConversationStreamIter`] of run-progress events.
    pub fn conversation_streamed(
        &self,
        agent_id: impl Into<String> + Send + 'static,
        query: impl Into<String> + Send + 'static,
        chat_uid: Option<String>,
    ) -> Result<agent::ConversationStreamIter> {
        let stream = self.rt.call(move |ctx| async move {
            // Box the RPIT stream so it can flow through `rt.call`'s generic
            // `R: Send + 'static`.
            Ok(
                Box::pin(ctx.conversation_streamed(agent_id, query, chat_uid).await?)
                    as Pin<Box<dyn Stream<Item = Result<ConversationStreamEvent>> + Send>>,
            )
        })?;
        Ok(agent::conversation_stream_iter(stream))
    }

    /// Resume an interrupted conversation, returning a blocking
    /// [`agent::ConversationStreamIter`] of run-progress events.
    pub fn continue_conversation_streamed(
        &self,
        agent_id: impl Into<String> + Send + 'static,
        chat_uid: impl Into<String> + Send + 'static,
        message_id: impl Into<String> + Send + 'static,
        answers: AnswersByToolCall,
    ) -> Result<agent::ConversationStreamIter> {
        let stream = self.rt.call(move |ctx| async move {
            Ok(Box::pin(
                ctx.continue_conversation_streamed(agent_id, chat_uid, message_id, answers)
                    .await?,
            )
                as Pin<
                    Box<dyn Stream<Item = Result<ConversationStreamEvent>> + Send>,
                >)
        })?;
        Ok(agent::conversation_stream_iter(stream))
    }
}
