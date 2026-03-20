use longbridge_python_macros::PyObject;
use pyo3::prelude::*;

use crate::time::PyOffsetDateTimeWrapper;

/// Topic item
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::TopicItem")]
pub(crate) struct TopicItem {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    published_at: PyOffsetDateTimeWrapper,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}

/// News item
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::content::NewsItem")]
pub(crate) struct NewsItem {
    /// News ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    published_at: PyOffsetDateTimeWrapper,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}
