use std::os::raw::c_char;

use longbridge::sharelist::{
    SharelistDetail, SharelistInfo, SharelistList, SharelistScopes, SharelistStock,
};

use crate::types::{CString, CVec, ToFFI};

/// A stock entry within a sharelist.
#[repr(C)]
pub struct CSharelistStock {
    /// Stock symbol (e.g. "AAPL.US").
    pub symbol: *const c_char,
    /// Display name of the stock.
    pub name: *const c_char,
    /// Market code (e.g. "US", "HK").
    pub market: *const c_char,
    /// Stock code (ticker without market suffix).
    pub code: *const c_char,
    /// Short introduction or description of the stock.
    pub intro: *const c_char,
    /// Category of unread change log entries for this stock.
    pub unread_change_log_category: *const c_char,
    /// Price change amount (decimal string); null if not available.
    pub change: *const c_char,
    /// Last traded price (decimal string); null if not available.
    pub last_done: *const c_char,
    /// Trade status code; valid only when `has_trade_status` is true.
    pub trade_status: i32,
    /// Whether `trade_status` contains a valid value.
    pub has_trade_status: bool,
}

pub(crate) struct CSharelistStockOwned {
    symbol: CString,
    name: CString,
    market: CString,
    code: CString,
    intro: CString,
    unread_change_log_category: CString,
    change: Option<CString>,
    last_done: Option<CString>,
    trade_status: i32,
    has_trade_status: bool,
}

impl From<SharelistStock> for CSharelistStockOwned {
    fn from(v: SharelistStock) -> Self {
        let (ts, has_ts) = match v.trade_status {
            Some(s) => (s, true),
            None => (0, false),
        };
        Self {
            symbol: v.symbol.into(),
            name: v.name.into(),
            market: v.market.into(),
            code: v.code.into(),
            intro: v.intro.into(),
            unread_change_log_category: v.unread_change_log_category.into(),
            change: v.change.map(|d| CString::from(d.to_string())),
            last_done: v.last_done.map(|d| CString::from(d.to_string())),
            trade_status: ts,
            has_trade_status: has_ts,
        }
    }
}

impl ToFFI for CSharelistStockOwned {
    type FFIType = CSharelistStock;
    fn to_ffi_type(&self) -> Self::FFIType {
        CSharelistStock {
            symbol: self.symbol.to_ffi_type(),
            name: self.name.to_ffi_type(),
            market: self.market.to_ffi_type(),
            code: self.code.to_ffi_type(),
            intro: self.intro.to_ffi_type(),
            unread_change_log_category: self.unread_change_log_category.to_ffi_type(),
            change: self
                .change
                .as_ref()
                .map(|s| s.to_ffi_type())
                .unwrap_or(std::ptr::null()),
            last_done: self
                .last_done
                .as_ref()
                .map(|s| s.to_ffi_type())
                .unwrap_or(std::ptr::null()),
            trade_status: self.trade_status,
            has_trade_status: self.has_trade_status,
        }
    }
}

/// Access/permission scopes associated with a sharelist.
#[repr(C)]
pub struct CSharelistScopes {
    /// Whether the current user is subscribed to this sharelist.
    pub subscription: bool,
    /// Whether this sharelist was created by the current authenticated user.
    pub is_self: bool,
}

pub(crate) struct CSharelistScopesOwned {
    subscription: bool,
    is_self: bool,
}

impl From<SharelistScopes> for CSharelistScopesOwned {
    fn from(v: SharelistScopes) -> Self {
        Self {
            subscription: v.subscription,
            is_self: v.is_self,
        }
    }
}

impl ToFFI for CSharelistScopesOwned {
    type FFIType = CSharelistScopes;
    fn to_ffi_type(&self) -> Self::FFIType {
        CSharelistScopes {
            subscription: self.subscription,
            is_self: self.is_self,
        }
    }
}

/// Summary information about a sharelist.
#[repr(C)]
pub struct CSharelistInfo {
    /// Unique sharelist identifier.
    pub id: i64,
    /// Display name of the sharelist.
    pub name: *const c_char,
    /// Human-readable description of the sharelist.
    pub description: *const c_char,
    /// URL of the cover image for the sharelist.
    pub cover: *const c_char,
    /// Total number of subscribers.
    pub subscribers_count: i64,
    /// Creation timestamp (Unix seconds).
    pub created_at: i64,
    /// Last-edited timestamp (Unix seconds).
    pub edited_at: i64,
    /// Year-to-date price change percentage (decimal string).
    pub this_year_chg: *const c_char,
    /// Creator information serialised as a JSON string.
    pub creator: *const c_char,
    /// Pointer to the array of stocks in this sharelist.
    pub stocks: *const CSharelistStock,
    /// Number of stocks in the array.
    pub num_stocks: usize,
    /// Whether the current user has subscribed to this sharelist.
    pub subscribed: bool,
    /// Overall price change percentage of the sharelist (decimal string).
    pub chg: *const c_char,
    /// Type code of the sharelist (e.g. 0 = normal, 1 = industry, …).
    pub sharelist_type: i32,
    /// Industry code associated with the sharelist (if applicable).
    pub industry_code: *const c_char,
}

pub(crate) struct CSharelistInfoOwned {
    id: i64,
    name: CString,
    description: CString,
    cover: CString,
    subscribers_count: i64,
    created_at: i64,
    edited_at: i64,
    this_year_chg: CString,
    creator: CString,
    stocks: CVec<CSharelistStockOwned>,
    subscribed: bool,
    chg: CString,
    sharelist_type: i32,
    industry_code: CString,
}

impl From<SharelistInfo> for CSharelistInfoOwned {
    fn from(v: SharelistInfo) -> Self {
        Self {
            id: v.id,
            name: v.name.into(),
            description: v.description.into(),
            cover: v.cover.into(),
            subscribers_count: v.subscribers_count,
            created_at: v.created_at.unix_timestamp(),
            edited_at: v.edited_at.unix_timestamp(),
            this_year_chg: v
                .this_year_chg
                .map(|d| d.to_string())
                .unwrap_or_default()
                .into(),
            creator: serde_json::to_string(&v.creator).unwrap_or_default().into(),
            stocks: v.stocks.into(),
            subscribed: v.subscribed,
            chg: v.chg.map(|d| d.to_string()).unwrap_or_default().into(),
            sharelist_type: v.sharelist_type,
            industry_code: v.industry_code.into(),
        }
    }
}

impl ToFFI for CSharelistInfoOwned {
    type FFIType = CSharelistInfo;
    fn to_ffi_type(&self) -> Self::FFIType {
        CSharelistInfo {
            id: self.id,
            name: self.name.to_ffi_type(),
            description: self.description.to_ffi_type(),
            cover: self.cover.to_ffi_type(),
            subscribers_count: self.subscribers_count,
            created_at: self.created_at,
            edited_at: self.edited_at,
            this_year_chg: self.this_year_chg.to_ffi_type(),
            creator: self.creator.to_ffi_type(),
            stocks: self.stocks.to_ffi_type(),
            num_stocks: self.stocks.len(),
            subscribed: self.subscribed,
            chg: self.chg.to_ffi_type(),
            sharelist_type: self.sharelist_type,
            industry_code: self.industry_code.to_ffi_type(),
        }
    }
}

/// Paginated list of sharelists with subscription information.
#[repr(C)]
pub struct CSharelistList {
    /// Pointer to the array of all sharelists.
    pub sharelists: *const CSharelistInfo,
    /// Number of sharelists in the array.
    pub num_sharelists: usize,
    /// Pointer to the array of sharelists the current user has subscribed to.
    pub subscribed_sharelists: *const CSharelistInfo,
    /// Number of subscribed sharelists in the array.
    pub num_subscribed_sharelists: usize,
    /// Pagination cursor for fetching the next page of results.
    pub tail_mark: *const c_char,
}

pub(crate) struct CSharelistListOwned {
    sharelists: CVec<CSharelistInfoOwned>,
    subscribed_sharelists: CVec<CSharelistInfoOwned>,
    tail_mark: CString,
}

impl From<SharelistList> for CSharelistListOwned {
    fn from(v: SharelistList) -> Self {
        Self {
            sharelists: v.sharelists.into(),
            subscribed_sharelists: v.subscribed_sharelists.into(),
            tail_mark: v.tail_mark.into(),
        }
    }
}

impl ToFFI for CSharelistListOwned {
    type FFIType = CSharelistList;
    fn to_ffi_type(&self) -> Self::FFIType {
        CSharelistList {
            sharelists: self.sharelists.to_ffi_type(),
            num_sharelists: self.sharelists.len(),
            subscribed_sharelists: self.subscribed_sharelists.to_ffi_type(),
            num_subscribed_sharelists: self.subscribed_sharelists.len(),
            tail_mark: self.tail_mark.to_ffi_type(),
        }
    }
}

/// Full detail of a sharelist including access scopes.
#[repr(C)]
pub struct CSharelistDetail {
    /// Sharelist summary information.
    pub sharelist: CSharelistInfo,
    /// Access/permission scopes for the current user relative to this
    /// sharelist.
    pub scopes: CSharelistScopes,
}

pub(crate) struct CSharelistDetailOwned {
    sharelist: CSharelistInfoOwned,
    scopes: CSharelistScopesOwned,
}

impl From<SharelistDetail> for CSharelistDetailOwned {
    fn from(v: SharelistDetail) -> Self {
        Self {
            sharelist: v.sharelist.into(),
            scopes: v.scopes.into(),
        }
    }
}

impl ToFFI for CSharelistDetailOwned {
    type FFIType = CSharelistDetail;
    fn to_ffi_type(&self) -> Self::FFIType {
        CSharelistDetail {
            sharelist: self.sharelist.to_ffi_type(),
            scopes: self.scopes.to_ffi_type(),
        }
    }
}
