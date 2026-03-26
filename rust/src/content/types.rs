use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::serde_utils;

/// Topic author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicAuthor {
    /// Member ID
    #[serde(default)]
    pub member_id: String,
    /// Display name
    #[serde(default)]
    pub name: String,
    /// Avatar URL
    #[serde(default)]
    pub avatar: String,
}

/// Topic image
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicImage {
    /// Original image URL
    #[serde(default)]
    pub url: String,
    /// Small thumbnail URL
    #[serde(default)]
    pub sm: String,
    /// Large image URL
    #[serde(default)]
    pub lg: String,
}

/// My topic item (topic created by the current authenticated user)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnedTopic {
    /// Topic ID
    pub id: String,
    /// Title
    #[serde(default)]
    pub title: String,
    /// Plain text excerpt
    #[serde(default)]
    pub description: String,
    /// Markdown body
    #[serde(default)]
    pub body: String,
    /// Author
    pub author: TopicAuthor,
    /// Related stock tickers, format: {symbol}.{market}
    #[serde(default)]
    pub tickers: Vec<String>,
    /// Hashtag names
    #[serde(default)]
    pub hashtags: Vec<String>,
    /// Images
    #[serde(default)]
    pub images: Vec<TopicImage>,
    /// Likes count
    #[serde(default)]
    pub likes_count: i32,
    /// Comments count
    #[serde(default)]
    pub comments_count: i32,
    /// Views count
    #[serde(default)]
    pub views_count: i32,
    /// Shares count
    #[serde(default)]
    pub shares_count: i32,
    /// Content type: "article" or "post"
    #[serde(default)]
    pub topic_type: String,
    /// URL to the full topic page
    #[serde(default)]
    pub detail_url: String,
    /// Created time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub created_at: OffsetDateTime,
    /// Updated time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub updated_at: OffsetDateTime,
}

/// Options for listing topics created by the current authenticated user
#[derive(Debug, Default, Clone, Serialize)]
pub struct MyTopicsOptions {
    /// Page number (default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Records per page, range 1~500 (default 50)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Filter by topic type: "article" or "post"; empty returns all
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_type: Option<String>,
}

/// Options for creating a topic
#[derive(Debug, Clone, Serialize)]
pub struct CreateTopicOptions {
    /// Topic title (required)
    pub title: String,
    /// Topic body in Markdown format (required)
    pub body: String,
    /// Content type: "article" (long-form) or "post" (short post, default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_type: Option<String>,
    /// Related stock tickers, format: {symbol}.{market}, max 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tickers: Option<Vec<String>>,
    /// Hashtag names, max 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<String>>,
}

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

/// Options for listing replies on a topic
#[derive(Debug, Default, Clone, Serialize)]
pub struct ListTopicRepliesOptions {
    /// Page number (default 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Records per page, range 1~50 (default 20)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

/// Options for posting a reply to a topic
#[derive(Debug, Clone, Serialize)]
pub struct CreateReplyOptions {
    /// Reply body. Plain text only — Markdown is not rendered.
    ///
    /// Stock symbols mentioned in the body (e.g. `700.HK`, `TSLA.US`) are
    /// automatically recognized and linked as related stocks by the platform.
    /// Use `tickers` in the parent topic to associate additional stocks not
    /// mentioned in the body.
    pub body: String,
    /// ID of the reply to. Set to `None` to post a top-level reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<String>,
}

/// A reply on a topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicReply {
    /// Reply ID
    pub id: String,
    /// Topic ID this reply belongs to
    pub topic_id: String,
    /// Reply body (plain text)
    #[serde(default)]
    pub body: String,
    /// ID of the parent reply (`"0"` means top-level)
    #[serde(default)]
    pub reply_to_id: String,
    /// Author info
    pub author: TopicAuthor,
    /// Attached images
    #[serde(default)]
    pub images: Vec<TopicImage>,
    /// Likes count
    #[serde(default)]
    pub likes_count: i32,
    /// Nested replies count
    #[serde(default)]
    pub comments_count: i32,
    /// Created time
    #[serde(
        serialize_with = "time::serde::rfc3339::serialize",
        deserialize_with = "serde_utils::timestamp::deserialize"
    )]
    pub created_at: OffsetDateTime,
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
