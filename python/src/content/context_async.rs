use std::sync::Arc;

use longbridge::content::{
    ContentContext, CreateReplyOptions, CreateTopicOptions, ListTopicRepliesOptions,
    MyTopicsOptions,
};
use pyo3::{prelude::*, types::PyType};

use crate::{
    config::Config,
    content::types::{NewsItem, OwnedTopic, TopicItem, TopicReply},
    error::ErrorNewType,
};

/// Async content context.
#[pyclass]
pub(crate) struct AsyncContentContext {
    ctx: Arc<ContentContext>,
}

#[pymethods]
impl AsyncContentContext {
    /// Create an async content context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        AsyncContentContext {
            ctx: Arc::new(ContentContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get topics created by the current authenticated user. Returns awaitable.
    #[pyo3(signature = (page = None, size = None, topic_type = None))]
    fn my_topics(
        &self,
        py: Python<'_>,
        page: Option<i32>,
        size: Option<i32>,
        topic_type: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .my_topics(MyTopicsOptions {
                    page,
                    size,
                    topic_type,
                })
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<OwnedTopic> { x.try_into() })
                .collect::<PyResult<Vec<OwnedTopic>>>()
        })
        .map(|b| b.unbind())
    }

    /// Create a new community topic. Returns awaitable.
    ///
    /// See: <https://open.longbridge.com/docs/api?op=create_topic>
    #[pyo3(signature = (title, body, topic_type = None, tickers = None, hashtags = None))]
    fn create_topic(
        &self,
        py: Python<'_>,
        title: String,
        body: String,
        topic_type: Option<String>,
        tickers: Option<Vec<String>>,
        hashtags: Option<Vec<String>>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            Ok(ctx
                .create_topic(CreateTopicOptions {
                    title,
                    body,
                    topic_type,
                    tickers,
                    hashtags,
                })
                .await
                .map_err(ErrorNewType)?)
        })
        .map(|b| b.unbind())
    }

    /// Get discussion topics list. Returns awaitable.
    fn topics(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.topics(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<TopicItem> { x.try_into() })
                .collect::<PyResult<Vec<TopicItem>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get news list. Returns awaitable.
    fn news(&self, py: Python<'_>, symbol: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.news(symbol).await.map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<NewsItem> { x.try_into() })
                .collect::<PyResult<Vec<NewsItem>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get full details of a topic by its ID. Returns awaitable.
    fn topic_detail(&self, py: Python<'_>, id: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx.topic_detail(id).await.map_err(ErrorNewType)?;
            OwnedTopic::try_from(v)
        })
        .map(|b| b.unbind())
    }

    /// List replies on a topic. Returns awaitable.
    #[pyo3(signature = (topic_id, page = None, size = None))]
    fn list_topic_replies(
        &self,
        py: Python<'_>,
        topic_id: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .list_topic_replies(topic_id, ListTopicRepliesOptions { page, size })
                .await
                .map_err(ErrorNewType)?;
            v.into_iter()
                .map(|x| -> PyResult<TopicReply> { x.try_into() })
                .collect::<PyResult<Vec<TopicReply>>>()
        })
        .map(|b| b.unbind())
    }

    /// Post a reply to a community topic. Returns awaitable.
    ///
    /// See: <https://open.longbridge.com/docs/api?op=create_topic_reply>
    #[pyo3(signature = (topic_id, body, reply_to_id = None))]
    fn create_topic_reply(
        &self,
        py: Python<'_>,
        topic_id: String,
        body: String,
        reply_to_id: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v = ctx
                .create_topic_reply(topic_id, CreateReplyOptions { body, reply_to_id })
                .await
                .map_err(ErrorNewType)?;
            TopicReply::try_from(v)
        })
        .map(|b| b.unbind())
    }
}
