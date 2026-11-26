use chrono::{DateTime, Utc};

use crate::{
    asset::types::{DepositState, RewardType},
    utils::from_datetime,
};

/// Statement type enum
#[napi_derive::napi]
#[derive(Debug, Clone, Copy)]
pub enum StatementType {
    /// Daily statement
    Daily = 1,
    /// Monthly statement
    Monthly = 2,
}

impl From<StatementType> for longbridge::asset::StatementType {
    fn from(value: StatementType) -> Self {
        match value {
            StatementType::Daily => longbridge::asset::StatementType::Daily,
            StatementType::Monthly => longbridge::asset::StatementType::Monthly,
        }
    }
}

/// Options for listing statements
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetStatementListRequest {
    /// Statement type: Daily (1) or Monthly (2)
    pub statement_type: Option<StatementType>,
    /// Start date for pagination
    pub start_date: Option<i32>,
    /// Number of results (default 20)
    pub limit: Option<i32>,
}

/// Options for getting a statement download URL
#[napi_derive::napi(object)]
#[derive(Debug)]
pub struct GetStatementDownloadUrlRequest {
    /// File key obtained from the list statements endpoint
    pub file_key: String,
}

/// Options for the stock transfer-in and transfer-out lists
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetStockTransfersOptions {
    /// Filter by market (`HK`, `US`, `CN`, `SG`)
    pub market: Option<String>,
    /// Filter by state code: a `StockTransferInState` code for the
    /// transfer-in list, a `StockTransferOutState` code for the transfer-out
    /// list (see the Rust SDK for the numeric codes)
    pub state: Option<i32>,
    /// Only applications created at or after this time
    pub start: Option<DateTime<Utc>>,
    /// Only applications created at or before this time
    pub end: Option<DateTime<Utc>>,
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Page size (default 20, max 50)
    pub size: Option<i32>,
}

impl From<GetStockTransfersOptions> for longbridge::asset::GetStockTransfersOptions {
    fn from(opts: GetStockTransfersOptions) -> Self {
        let mut opts2 = longbridge::asset::GetStockTransfersOptions::new();
        if let Some(market) = opts.market {
            opts2 = opts2.market(market);
        }
        if let Some(state) = opts.state {
            opts2 = opts2.state(state);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(from_datetime(start));
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(from_datetime(end));
        }
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        opts2
    }
}

/// Options for the reward record list
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetRewardRecordsOptions {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Page size (max 50)
    pub page_size: Option<i32>,
    /// Filter by reward type (default: all types)
    pub reward_type: Option<RewardType>,
}

impl From<GetRewardRecordsOptions> for longbridge::asset::GetRewardRecordsOptions {
    fn from(opts: GetRewardRecordsOptions) -> Self {
        let mut opts2 = longbridge::asset::GetRewardRecordsOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(page_size) = opts.page_size {
            opts2 = opts2.page_size(page_size);
        }
        if let Some(reward_type) = opts.reward_type {
            opts2 = opts2.reward_type(reward_type.into());
        }
        opts2
    }
}

/// Options for the coupon list
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetCouponsOptions {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Page size (max 50)
    pub page_size: Option<i32>,
    /// List expired coupons instead of the ones that have not expired
    /// (default false)
    pub expired: Option<bool>,
    /// Filter by coupon type (e.g. `commission`, `platform_fee_deduct`,
    /// `stock`, `quote`, `cash`; default: all types)
    pub card_type: Option<String>,
}

impl From<GetCouponsOptions> for longbridge::asset::GetCouponsOptions {
    fn from(opts: GetCouponsOptions) -> Self {
        let mut opts2 = longbridge::asset::GetCouponsOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(page_size) = opts.page_size {
            opts2 = opts2.page_size(page_size);
        }
        if let Some(expired) = opts.expired {
            opts2 = opts2.expired(expired);
        }
        if let Some(card_type) = opts.card_type {
            opts2 = opts2.card_type(card_type);
        }
        opts2
    }
}

/// Options for the deposit list
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetDepositsOptions {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
    /// Filter by states (`Unknown` is ignored)
    pub states: Option<Vec<DepositState>>,
    /// Filter by currencies (e.g. `HKD`, `USD`)
    pub currencies: Option<Vec<String>>,
    /// Only deposits applied at or after this time
    pub start: Option<DateTime<Utc>>,
    /// Only deposits applied at or before this time
    pub end: Option<DateTime<Utc>>,
}

impl From<GetDepositsOptions> for longbridge::asset::GetDepositsOptions {
    fn from(opts: GetDepositsOptions) -> Self {
        let mut opts2 = longbridge::asset::GetDepositsOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        if let Some(states) = opts.states {
            opts2 = opts2.states(states.into_iter().map(Into::into));
        }
        if let Some(currencies) = opts.currencies {
            opts2 = opts2.currencies(currencies);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(from_datetime(start));
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(from_datetime(end));
        }
        opts2
    }
}

/// Options for the withdrawal list
#[napi_derive::napi(object)]
#[derive(Debug, Default)]
pub struct GetWithdrawalsOptions {
    /// Page number (default 1)
    pub page: Option<i32>,
    /// Page size
    pub size: Option<i32>,
    /// Only withdrawals applied at or after this time
    pub start: Option<DateTime<Utc>>,
    /// Only withdrawals applied at or before this time
    pub end: Option<DateTime<Utc>>,
}

impl From<GetWithdrawalsOptions> for longbridge::asset::GetWithdrawalsOptions {
    fn from(opts: GetWithdrawalsOptions) -> Self {
        let mut opts2 = longbridge::asset::GetWithdrawalsOptions::new();
        if let Some(page) = opts.page {
            opts2 = opts2.page(page);
        }
        if let Some(size) = opts.size {
            opts2 = opts2.size(size);
        }
        if let Some(start) = opts.start {
            opts2 = opts2.start(from_datetime(start));
        }
        if let Some(end) = opts.end {
            opts2 = opts2.end(from_datetime(end));
        }
        opts2
    }
}
