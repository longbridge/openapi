use std::os::raw::c_char;

use longbridge::content::{NewsItem, OwnedTopic, TopicAuthor, TopicImage, TopicItem};

use crate::types::{CString, CVec, ToFFI};

/// Topic author
#[repr(C)]
pub struct CTopicAuthor {
    /// Member ID
    pub member_id: *const c_char,
    /// Display name
    pub name: *const c_char,
    /// Avatar URL
    pub avatar: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CTopicAuthorOwned {
    member_id: CString,
    name: CString,
    avatar: CString,
}

impl From<TopicAuthor> for CTopicAuthorOwned {
    fn from(a: TopicAuthor) -> Self {
        Self {
            member_id: a.member_id.into(),
            name: a.name.into(),
            avatar: a.avatar.into(),
        }
    }
}

impl ToFFI for CTopicAuthorOwned {
    type FFIType = CTopicAuthor;
    fn to_ffi_type(&self) -> CTopicAuthor {
        CTopicAuthor {
            member_id: self.member_id.to_ffi_type(),
            name: self.name.to_ffi_type(),
            avatar: self.avatar.to_ffi_type(),
        }
    }
}

/// Topic image
#[repr(C)]
pub struct CTopicImage {
    /// Original image URL
    pub url: *const c_char,
    /// Small thumbnail URL
    pub sm: *const c_char,
    /// Large image URL
    pub lg: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CTopicImageOwned {
    url: CString,
    sm: CString,
    lg: CString,
}

impl From<TopicImage> for CTopicImageOwned {
    fn from(img: TopicImage) -> Self {
        Self {
            url: img.url.into(),
            sm: img.sm.into(),
            lg: img.lg.into(),
        }
    }
}

impl ToFFI for CTopicImageOwned {
    type FFIType = CTopicImage;
    fn to_ffi_type(&self) -> CTopicImage {
        CTopicImage {
            url: self.url.to_ffi_type(),
            sm: self.sm.to_ffi_type(),
            lg: self.lg.to_ffi_type(),
        }
    }
}

/// My topic item (topic created by the current authenticated user)
#[repr(C)]
pub struct COwnedTopic {
    /// Topic ID
    pub id: *const c_char,
    /// Title
    pub title: *const c_char,
    /// Plain text excerpt
    pub description: *const c_char,
    /// Markdown body
    pub body: *const c_char,
    /// Author
    pub author: CTopicAuthor,
    /// Related stock tickers
    pub tickers: *const *const c_char,
    /// Number of tickers
    pub num_tickers: usize,
    /// Hashtag names
    pub hashtags: *const *const c_char,
    /// Number of hashtags
    pub num_hashtags: usize,
    /// Images
    pub images: *const CTopicImage,
    /// Number of images
    pub num_images: usize,
    /// Likes count
    pub likes_count: i32,
    /// Comments count
    pub comments_count: i32,
    /// Views count
    pub views_count: i32,
    /// Shares count
    pub shares_count: i32,
    /// Content type: "article" or "post"
    pub topic_type: *const c_char,
    /// License: 0=none, 1=original, 2=non-original
    pub license: i32,
    /// URL to the full topic page
    pub detail_url: *const c_char,
    /// Created time (Unix timestamp)
    pub created_at: i64,
    /// Updated time (Unix timestamp)
    pub updated_at: i64,
}

#[derive(Debug)]
pub(crate) struct COwnedTopicOwned {
    id: CString,
    title: CString,
    description: CString,
    body: CString,
    author: CTopicAuthorOwned,
    tickers: CVec<CString>,
    hashtags: CVec<CString>,
    images: CVec<CTopicImageOwned>,
    likes_count: i32,
    comments_count: i32,
    views_count: i32,
    shares_count: i32,
    topic_type: CString,
    license: i32,
    detail_url: CString,
    created_at: i64,
    updated_at: i64,
}

impl From<OwnedTopic> for COwnedTopicOwned {
    fn from(item: OwnedTopic) -> Self {
        Self {
            id: item.id.into(),
            title: item.title.into(),
            description: item.description.into(),
            body: item.body.into(),
            author: item.author.into(),
            tickers: item.tickers.into(),
            hashtags: item.hashtags.into(),
            images: item.images.into(),
            likes_count: item.likes_count,
            comments_count: item.comments_count,
            views_count: item.views_count,
            shares_count: item.shares_count,
            topic_type: item.topic_type.into(),
            license: item.license,
            detail_url: item.detail_url.into(),
            created_at: item.created_at.unix_timestamp(),
            updated_at: item.updated_at.unix_timestamp(),
        }
    }
}

impl ToFFI for COwnedTopicOwned {
    type FFIType = COwnedTopic;
    fn to_ffi_type(&self) -> COwnedTopic {
        COwnedTopic {
            id: self.id.to_ffi_type(),
            title: self.title.to_ffi_type(),
            description: self.description.to_ffi_type(),
            body: self.body.to_ffi_type(),
            author: self.author.to_ffi_type(),
            tickers: self.tickers.to_ffi_type(),
            num_tickers: self.tickers.len(),
            hashtags: self.hashtags.to_ffi_type(),
            num_hashtags: self.hashtags.len(),
            images: self.images.to_ffi_type(),
            num_images: self.images.len(),
            likes_count: self.likes_count,
            comments_count: self.comments_count,
            views_count: self.views_count,
            shares_count: self.shares_count,
            topic_type: self.topic_type.to_ffi_type(),
            license: self.license,
            detail_url: self.detail_url.to_ffi_type(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

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
