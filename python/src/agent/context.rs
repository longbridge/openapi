use std::{collections::HashMap, sync::Arc};

use longbridge::{
    agent::{self, GetAgentsOptions},
    blocking::AgentContextSync,
};
use parking_lot::Mutex;
use pyo3::prelude::*;

use crate::{
    agent::types::{
        AgentsResponse, ConversationResponse, ConversationStreamEvent, WorkspacesResponse,
    },
    config::Config,
    error::ErrorNewType,
};

/// AI Agent conversation context.
#[pyclass]
pub(crate) struct AgentContext(AgentContextSync);

#[pymethods]
impl AgentContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self(
            AgentContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        ))
    }

    /// List the Workspaces the current account belongs to.
    fn workspaces(&self, py: Python<'_>) -> PyResult<WorkspacesResponse> {
        Ok(py
            .detach(|| self.0.workspaces())
            .map_err(ErrorNewType)?
            .into())
    }

    /// List the Agents in the specified Workspace.
    #[pyo3(signature = (workspace_id, page = None, limit = None, name = None))]
    fn agents(
        &self,
        py: Python<'_>,
        workspace_id: String,
        page: Option<i32>,
        limit: Option<i32>,
        name: Option<String>,
    ) -> PyResult<AgentsResponse> {
        let mut opts = GetAgentsOptions::new();
        if let Some(page) = page {
            opts = opts.page(page);
        }
        if let Some(limit) = limit {
            opts = opts.limit(limit);
        }
        if let Some(name) = name {
            opts = opts.name(name);
        }

        Ok(py
            .detach(|| self.0.agents(workspace_id, Some(opts)))
            .map_err(ErrorNewType)?
            .into())
    }

    /// Start a conversation with the specified Agent, blocking until the run
    /// succeeds, is interrupted, or fails.
    #[pyo3(signature = (agent_id, query, chat_uid = None))]
    fn conversation(
        &self,
        py: Python<'_>,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
    ) -> PyResult<ConversationResponse> {
        Ok(py
            .detach(|| self.0.conversation(agent_id, query, chat_uid))
            .map_err(ErrorNewType)?
            .into())
    }

    /// Resume an interrupted conversation, blocking until the run succeeds, is
    /// interrupted again, or fails.
    ///
    /// `answers_by_tool_call` maps `tool_call_id` (from
    /// `ConversationResponse.interrupt`) to a map of question text to answer.
    fn continue_conversation(
        &self,
        py: Python<'_>,
        agent_id: String,
        chat_uid: String,
        message_id: String,
        answers_by_tool_call: HashMap<String, HashMap<String, String>>,
    ) -> PyResult<ConversationResponse> {
        Ok(py
            .detach(|| {
                self.0
                    .continue_conversation(agent_id, chat_uid, message_id, answers_by_tool_call)
            })
            .map_err(ErrorNewType)?
            .into())
    }

    /// Start a conversation with the specified Agent, returning an iterator of
    /// run-progress events. The last item is always a `ConversationStreamEvent`
    /// with `kind == "workflow_finished"`.
    #[pyo3(signature = (agent_id, query, chat_uid = None))]
    fn conversation_streamed(
        &self,
        py: Python<'_>,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
    ) -> PyResult<ConversationStreamIter> {
        let iter = py
            .detach(|| self.0.conversation_streamed(agent_id, query, chat_uid))
            .map_err(ErrorNewType)?;
        Ok(ConversationStreamIter(Mutex::new(iter)))
    }

    /// Resume an interrupted conversation, returning an iterator of
    /// run-progress events.
    fn continue_conversation_streamed(
        &self,
        py: Python<'_>,
        agent_id: String,
        chat_uid: String,
        message_id: String,
        answers_by_tool_call: HashMap<String, HashMap<String, String>>,
    ) -> PyResult<ConversationStreamIter> {
        let iter = py
            .detach(|| {
                self.0.continue_conversation_streamed(
                    agent_id,
                    chat_uid,
                    message_id,
                    answers_by_tool_call,
                )
            })
            .map_err(ErrorNewType)?;
        Ok(ConversationStreamIter(Mutex::new(iter)))
    }
}

/// Blocking iterator of conversation-stream events, returned by
/// `AgentContext.conversation_streamed`/`continue_conversation_streamed`. Use
/// with a plain `for` loop.
///
/// Wrapped in a `Mutex` (rather than exposed as a bare
/// `agent::ConversationStreamIter`) because `#[pyclass]` requires its wrapped
/// type to be `Send + Sync`, and the inner `std::sync::mpsc::Receiver` is
/// `Send` but not `Sync`.
#[pyclass]
pub(crate) struct ConversationStreamIter(Mutex<agent::ConversationStreamIter>);

#[pymethods]
impl ConversationStreamIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self, py: Python<'_>) -> PyResult<Option<ConversationStreamEvent>> {
        match py.detach(|| self.0.lock().next()) {
            Some(Ok(event)) => Ok(Some(event.into())),
            Some(Err(err)) => Err(ErrorNewType(err).into()),
            None => Ok(None),
        }
    }
}
