use longbridge::sharelist::types as lb;

/// Stock in a sharelist
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SharelistStock {
    /// Security symbol
    pub symbol: String,
    /// Security name
    pub name: String,
    /// Market, e.g. `"HK"`
    pub market: String,
    /// Ticker code
    pub code: String,
    /// Brief description
    pub intro: String,
    /// Unread change log category
    pub unread_change_log_category: String,
    /// Day change percentage
    pub change: Option<String>,
    /// Latest price
    pub last_done: Option<String>,
    /// Trade status code
    pub trade_status: Option<i32>,
    /// Whether delayed quote
    pub latency: Option<bool>,
}
impl From<lb::SharelistStock> for SharelistStock {
    fn from(v: lb::SharelistStock) -> Self {
        Self {
            symbol: v.symbol,
            name: v.name,
            market: v.market,
            code: v.code,
            intro: v.intro,
            unread_change_log_category: v.unread_change_log_category,
            change: v.change.map(|d| d.to_string()),
            last_done: v.last_done.map(|d| d.to_string()),
            trade_status: v.trade_status,
            latency: v.latency,
        }
    }
}

/// Sharelist subscription scopes
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SharelistScopes {
    /// Whether the current user is subscribed
    pub subscription: bool,
    /// Whether the current user is the creator
    pub is_self: bool,
}
impl From<lb::SharelistScopes> for SharelistScopes {
    fn from(v: lb::SharelistScopes) -> Self {
        Self {
            subscription: v.subscription,
            is_self: v.is_self,
        }
    }
}

/// Sharelist information
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SharelistInfo {
    /// Sharelist ID
    pub id: i64,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Cover image URL
    pub cover: String,
    /// Number of subscribers
    pub subscribers_count: i64,
    /// Creation time (unix timestamp)
    pub created_at: i64,
    /// Last stock edit time (unix timestamp)
    pub edited_at: i64,
    /// YTD change percentage
    pub this_year_chg: Option<String>,
    /// Creator info
    pub creator: serde_json::Value,
    /// Constituent stocks
    pub stocks: Vec<SharelistStock>,
    /// Whether the current user is subscribed
    pub subscribed: bool,
    /// Day change percentage
    pub chg: Option<String>,
    /// Sharelist type: 0=regular, 3=official, 4=industry
    pub sharelist_type: i32,
    /// Industry code (for industry sharelists)
    pub industry_code: String,
}
impl From<lb::SharelistInfo> for SharelistInfo {
    fn from(v: lb::SharelistInfo) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            cover: v.cover,
            subscribers_count: v.subscribers_count,
            created_at: v.created_at.unix_timestamp(),
            edited_at: v.edited_at.unix_timestamp(),
            this_year_chg: v.this_year_chg.map(|d| d.to_string()),
            creator: v.creator,
            stocks: v.stocks.into_iter().map(Into::into).collect(),
            subscribed: v.subscribed,
            chg: v.chg.map(|d| d.to_string()),
            sharelist_type: v.sharelist_type,
            industry_code: v.industry_code,
        }
    }
}

/// Response for sharelist list and popular queries
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SharelistList {
    /// User's own and followed sharelists
    pub sharelists: Vec<SharelistInfo>,
    /// Subscribed sharelists (may be absent in popular response)
    pub subscribed_sharelists: Vec<SharelistInfo>,
    /// Pagination cursor for subscribed list
    pub tail_mark: String,
}
impl From<lb::SharelistList> for SharelistList {
    fn from(v: lb::SharelistList) -> Self {
        Self {
            sharelists: v.sharelists.into_iter().map(Into::into).collect(),
            subscribed_sharelists: v
                .subscribed_sharelists
                .into_iter()
                .map(Into::into)
                .collect(),
            tail_mark: v.tail_mark,
        }
    }
}

/// Sharelist detail response
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct SharelistDetail {
    /// Sharelist info
    pub sharelist: SharelistInfo,
    /// Subscription scopes
    pub scopes: SharelistScopes,
}
impl From<lb::SharelistDetail> for SharelistDetail {
    fn from(v: lb::SharelistDetail) -> Self {
        Self {
            sharelist: v.sharelist.into(),
            scopes: v.scopes.into(),
        }
    }
}
