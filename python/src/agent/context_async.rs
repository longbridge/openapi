//! Async AI Agent context backed by longbridge's native async API.

use std::{collections::HashMap, pin::Pin, sync::Arc};

use futures_util::{Stream, StreamExt};
use longbridge::agent::{
    AgentContext, ConversationStreamEvent as RustConversationStreamEvent, GetAgentsOptions,
};
use pyo3::{exceptions::PyStopAsyncIteration, prelude::*, types::PyType};
use tokio::sync::Mutex;

use crate::{
    agent::types::{
        AgentsResponse, ConversationResponse, ConversationStreamEvent, WorkspacesResponse,
    },
    config::Config,
    error::ErrorNewType,
};

/// A boxed conversation-stream event stream, stored behind a `tokio::Mutex` so
/// `AsyncConversationStreamIter::__anext__` (which only gets `&self`, per the
/// async-iterator protocol) can still drive it one item at a time.
type BoxedEventStream =
    Pin<Box<dyn Stream<Item = longbridge::Result<RustConversationStreamEvent>> + Send>>;

/// Async AI Agent context. Create via `AsyncAgentContext.create(config)`
/// (synchronous, no await needed). Use in asyncio.
#[pyclass]
pub(crate) struct AsyncAgentContext {
    ctx: Arc<AgentContext>,
}

#[pymethods]
impl AsyncAgentContext {
    /// Create an async AI Agent context (synchronous, no await needed).
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        let config = Arc::new(config.0.clone());
        AsyncAgentContext {
            ctx: Arc::new(AgentContext::new(config)),
        }
    }

    /// List the Workspaces the current account belongs to. Returns awaitable.
    fn workspaces(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp: WorkspacesResponse = ctx.workspaces().await.map_err(ErrorNewType)?.into();
            Ok(resp)
        })
        .map(|b| b.unbind())
    }

    /// List the Agents in the specified Workspace. Returns awaitable.
    #[pyo3(signature = (workspace_id, page = None, limit = None, name = None))]
    fn agents(
        &self,
        py: Python<'_>,
        workspace_id: String,
        page: Option<i32>,
        limit: Option<i32>,
        name: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
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
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp: AgentsResponse = ctx
                .agents(workspace_id, Some(opts))
                .await
                .map_err(ErrorNewType)?
                .into();
            Ok(resp)
        })
        .map(|b| b.unbind())
    }

    /// Start a conversation with the specified Agent, blocking until the run
    /// succeeds, is interrupted, or fails. Returns awaitable.
    #[pyo3(signature = (agent_id, query, chat_uid = None))]
    fn conversation(
        &self,
        py: Python<'_>,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp: ConversationResponse = ctx
                .conversation(agent_id, query, chat_uid)
                .await
                .map_err(ErrorNewType)?
                .into();
            Ok(resp)
        })
        .map(|b| b.unbind())
    }

    /// Resume an interrupted conversation, blocking until the run succeeds, is
    /// interrupted again, or fails. Returns awaitable.
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
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let resp: ConversationResponse = ctx
                .continue_conversation(agent_id, chat_uid, message_id, answers_by_tool_call)
                .await
                .map_err(ErrorNewType)?
                .into();
            Ok(resp)
        })
        .map(|b| b.unbind())
    }

    /// Start a conversation with the specified Agent. Returns an awaitable
    /// that resolves to an `AsyncConversationStreamIter`; use `async for` on
    /// it to consume run-progress events.
    #[pyo3(signature = (agent_id, query, chat_uid = None))]
    fn conversation_streamed(
        &self,
        py: Python<'_>,
        agent_id: String,
        query: String,
        chat_uid: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let stream = ctx
                .conversation_streamed(agent_id, query, chat_uid)
                .await
                .map_err(ErrorNewType)?;
            let boxed: BoxedEventStream = Box::pin(stream);
            Ok(AsyncConversationStreamIter {
                stream: Arc::new(Mutex::new(boxed)),
            })
        })
        .map(|b| b.unbind())
    }

    /// Resume an interrupted conversation. Returns an awaitable that resolves
    /// to an `AsyncConversationStreamIter`; use `async for` on it to consume
    /// run-progress events.
    fn continue_conversation_streamed(
        &self,
        py: Python<'_>,
        agent_id: String,
        chat_uid: String,
        message_id: String,
        answers_by_tool_call: HashMap<String, HashMap<String, String>>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let stream = ctx
                .continue_conversation_streamed(
                    agent_id,
                    chat_uid,
                    message_id,
                    answers_by_tool_call,
                )
                .await
                .map_err(ErrorNewType)?;
            let boxed: BoxedEventStream = Box::pin(stream);
            Ok(AsyncConversationStreamIter {
                stream: Arc::new(Mutex::new(boxed)),
            })
        })
        .map(|b| b.unbind())
    }
}

/// Async iterator of conversation-stream events, returned (once awaited) by
/// `AsyncAgentContext.conversation_streamed`/`continue_conversation_streamed`.
/// Use with `async for`.
#[pyclass]
pub(crate) struct AsyncConversationStreamIter {
    stream: Arc<Mutex<BoxedEventStream>>,
}

#[pymethods]
impl AsyncConversationStreamIter {
    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    /// Pull the next event.
    ///
    /// Per Python's async-iterator protocol, ending iteration means *raising*
    /// `StopAsyncIteration` from the coroutine `__anext__` returns (not just
    /// returning `None` — unlike the sync `__next__`/`StopIteration` case,
    /// there is no automatic `Option<T>` -> exception conversion here because
    /// this method's Rust return type is the awaitable object itself
    /// (`Py<PyAny>`, built via `future_into_py`), not the eventual value the
    /// awaitable produces).
    fn __anext__(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let stream = self.stream.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let mut guard = stream.lock().await;
            let event = match guard.next().await {
                Some(item) => item.map_err(ErrorNewType)?,
                None => return Err(PyStopAsyncIteration::new_err(())),
            };
            Ok(ConversationStreamEvent::from(event))
        })
        .map(|b| b.unbind())
    }
}
