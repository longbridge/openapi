use std::sync::Arc;

use longbridge::blocking::ContentContextSync;
use pyo3::prelude::*;

use crate::{
    config::Config,
    content::types::{NewsItem, TopicItem},
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
            ctx: ContentContextSync::try_new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
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
