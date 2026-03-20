use std::os::raw::c_char;

use longbridge::content::{NewsItem, TopicItem};

use crate::types::{CString, ToFFI};

/// Topic item
#[repr(C)]
pub struct CTopicItem {
    /// Topic ID
    pub id: *const c_char,
    /// Title
    pub title: *const c_char,
    /// Description
    pub description: *const c_char,
    /// URL
    pub url: *const c_char,
    /// Published time (Unix timestamp)
    pub published_at: i64,
    /// Comments count
    pub comments_count: i32,
    /// Likes count
    pub likes_count: i32,
    /// Shares count
    pub shares_count: i32,
}

#[derive(Debug)]
pub(crate) struct CTopicItemOwned {
    id: CString,
    title: CString,
    description: CString,
    url: CString,
    published_at: i64,
    comments_count: i32,
    likes_count: i32,
    shares_count: i32,
}

impl From<TopicItem> for CTopicItemOwned {
    fn from(item: TopicItem) -> Self {
        let TopicItem {
            id,
            title,
            description,
            url,
            published_at,
            comments_count,
            likes_count,
            shares_count,
        } = item;
        CTopicItemOwned {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            url: url.into(),
            published_at: published_at.unix_timestamp(),
            comments_count,
            likes_count,
            shares_count,
        }
    }
}

impl ToFFI for CTopicItemOwned {
    type FFIType = CTopicItem;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CTopicItemOwned {
            id,
            title,
            description,
            url,
            published_at,
            comments_count,
            likes_count,
            shares_count,
        } = self;
        CTopicItem {
            id: id.to_ffi_type(),
            title: title.to_ffi_type(),
            description: description.to_ffi_type(),
            url: url.to_ffi_type(),
            published_at: *published_at,
            comments_count: *comments_count,
            likes_count: *likes_count,
            shares_count: *shares_count,
        }
    }
}

/// News item
#[repr(C)]
pub struct CNewsItem {
    /// News ID
    pub id: *const c_char,
    /// Title
    pub title: *const c_char,
    /// Description
    pub description: *const c_char,
    /// URL
    pub url: *const c_char,
    /// Published time (Unix timestamp)
    pub published_at: i64,
    /// Comments count
    pub comments_count: i32,
    /// Likes count
    pub likes_count: i32,
    /// Shares count
    pub shares_count: i32,
}

#[derive(Debug)]
pub(crate) struct CNewsItemOwned {
    id: CString,
    title: CString,
    description: CString,
    url: CString,
    published_at: i64,
    comments_count: i32,
    likes_count: i32,
    shares_count: i32,
}

impl From<NewsItem> for CNewsItemOwned {
    fn from(item: NewsItem) -> Self {
        let NewsItem {
            id,
            title,
            description,
            url,
            published_at,
            comments_count,
            likes_count,
            shares_count,
        } = item;
        CNewsItemOwned {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            url: url.into(),
            published_at: published_at.unix_timestamp(),
            comments_count,
            likes_count,
            shares_count,
        }
    }
}

impl ToFFI for CNewsItemOwned {
    type FFIType = CNewsItem;

    fn to_ffi_type(&self) -> Self::FFIType {
        let CNewsItemOwned {
            id,
            title,
            description,
            url,
            published_at,
            comments_count,
            likes_count,
            shares_count,
        } = self;
        CNewsItem {
            id: id.to_ffi_type(),
            title: title.to_ffi_type(),
            description: description.to_ffi_type(),
            url: url.to_ffi_type(),
            published_at: *published_at,
            comments_count: *comments_count,
            likes_count: *likes_count,
            shares_count: *shares_count,
        }
    }
}
