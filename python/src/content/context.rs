use std::sync::Arc;

use longbridge::{
    blocking::ContentContextSync,
    content::{CreateTopicOptions, ListMyTopicsOptions},
};
use pyo3::prelude::*;

use crate::{
    config::Config,
    content::types::{NewsItem, OwnedTopic, TopicItem},
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
    pub fn topics_mine(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        topic_type: Option<String>,
    ) -> PyResult<Vec<OwnedTopic>> {
        self.ctx
            .topics_mine(ListMyTopicsOptions {
                page,
                size,
                topic_type,
            })
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Create a new topic
    #[pyo3(signature = (title, body, topic_type = None, tickers = None, hashtags = None, license = None))]
    pub fn create_topic(
        &self,
        title: String,
        body: String,
        topic_type: Option<String>,
        tickers: Option<Vec<String>>,
        hashtags: Option<Vec<String>>,
        license: Option<i32>,
    ) -> PyResult<String> {
        Ok(self
            .ctx
            .create_topic(CreateTopicOptions {
                title,
                body,
                topic_type,
                tickers,
                hashtags,
                license,
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
}
