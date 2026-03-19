use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::serde_utils;

/// Topic item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicItem {
    /// Topic ID
    pub id: String,
    /// Title
    #[serde(default)]
    pub title: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// URL
    pub url: String,
    /// Published time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub published_at: OffsetDateTime,
    /// Comments count
    pub comments_count: i32,
    /// Likes count
    pub likes_count: i32,
    /// Shares count
    pub shares_count: i32,
}

/// News item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    /// News ID
    pub id: String,
    /// Title
    #[serde(default)]
    pub title: String,
    /// Description
    #[serde(default)]
    pub description: String,
    /// URL
    pub url: String,
    /// Published time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub published_at: OffsetDateTime,
    /// Comments count
    pub comments_count: i32,
    /// Likes count
    pub likes_count: i32,
    /// Shares count
    pub shares_count: i32,
}
