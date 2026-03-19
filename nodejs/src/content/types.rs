use chrono::{DateTime, Utc};
use longbridge_nodejs_macros::JsObject;

/// Topic item
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::content::TopicItem")]
pub struct TopicItem {
    /// Topic ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    #[js(datetime)]
    published_at: DateTime<Utc>,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}

/// News item
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::content::NewsItem")]
pub struct NewsItem {
    /// News ID
    id: String,
    /// Title
    title: String,
    /// Description
    description: String,
    /// URL
    url: String,
    /// Published time
    #[js(datetime)]
    published_at: DateTime<Utc>,
    /// Comments count
    comments_count: i32,
    /// Likes count
    likes_count: i32,
    /// Shares count
    shares_count: i32,
}
