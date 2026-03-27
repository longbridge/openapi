use std::sync::Arc;

use longbridge::{
    blocking::ContentContextSync,
    content::{CreateReplyOptions, CreateTopicOptions, ListTopicRepliesOptions, MyTopicsOptions},
};
use pyo3::prelude::*;

use crate::{
    config::Config,
    content::types::{NewsItem, OwnedTopic, TopicItem, TopicReply},
    error::ErrorNewType,
};

#[pyclass]
pub(crate) struct ContentContext {
    ctx: ContentContextSync,
}

#[pymethods]
impl ContentContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: ContentContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get topics created by the current authenticated user
    #[pyo3(signature = (page = None, size = None, topic_type = None))]
    pub fn my_topics(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        topic_type: Option<String>,
    ) -> PyResult<Vec<OwnedTopic>> {
        self.ctx
            .my_topics(MyTopicsOptions {
                page,
                size,
                topic_type,
            })
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Create a new community topic.
    ///
    /// See: <https://open.longbridge.com/docs/api?op=create_topic>
    #[pyo3(signature = (title, body, topic_type = None, tickers = None, hashtags = None))]
    pub fn create_topic(
        &self,
        title: String,
        body: String,
        topic_type: Option<String>,
        tickers: Option<Vec<String>>,
        hashtags: Option<Vec<String>>,
    ) -> PyResult<String> {
        Ok(self
            .ctx
            .create_topic(CreateTopicOptions {
                title,
                body,
                topic_type,
                tickers,
                hashtags,
            })
            .map_err(ErrorNewType)?)
    }

    /// Get discussion topics list
    pub fn topics(&self, symbol: String) -> PyResult<Vec<TopicItem>> {
        self.ctx
            .topics(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get news list
    pub fn news(&self, symbol: String) -> PyResult<Vec<NewsItem>> {
        self.ctx
            .news(symbol)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get full details of a topic by its ID
    pub fn topic_detail(&self, id: String) -> PyResult<OwnedTopic> {
        self.ctx.topic_detail(id).map_err(ErrorNewType)?.try_into()
    }

    /// List replies on a topic
    #[pyo3(signature = (topic_id, page = None, size = None))]
    pub fn list_topic_replies(
        &self,
        topic_id: String,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<TopicReply>> {
        self.ctx
            .list_topic_replies(topic_id, ListTopicRepliesOptions { page, size })
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Post a reply to a community topic.
    ///
    /// See: <https://open.longbridge.com/docs/api?op=create_topic_reply>
    #[pyo3(signature = (topic_id, body, reply_to_id = None))]
    pub fn create_topic_reply(
        &self,
        topic_id: String,
        body: String,
        reply_to_id: Option<String>,
    ) -> PyResult<TopicReply> {
        self.ctx
            .create_topic_reply(topic_id, CreateReplyOptions { body, reply_to_id })
            .map_err(ErrorNewType)?
            .try_into()
    }
}
