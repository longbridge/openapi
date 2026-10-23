use chrono::{DateTime, Utc};

use crate::utils::from_datetime;

/// Options for the US transaction history request
#[napi_derive::napi(object, js_name = "GetUSTransactionHistoryOptions")]
pub struct GetUSTransactionHistoryOptions {
    /// Start time (inclusive)
    pub start_at: DateTime<Utc>,
    /// End time (inclusive)
    pub end_at: DateTime<Utc>,
    /// Page number, starting at 1 (default 1)
    pub page: Option<i32>,
    /// Page size (default 20)
    pub size: Option<i32>,
    /// Transaction types to include; empty means all. Values:
    /// `TT_DEPOSIT_AND_WITHDRAWAL`, `TT_STOCKS_AND_ETFS_TRADING`,
    /// `TT_OPTIONS_TRADING`, `TT_CRYPTO_TRADING`, `TT_CORPORATE_ACTION`,
    /// `TT_TRANSFER_IN_AND_OUT`, `TT_CRYPTO_TRANSFER`, `TT_MARGIN_FINANCING`,
    /// `TT_OTHER`
    pub types: Option<Vec<String>>,
    /// Currency filter, e.g. `USD`; empty means all
    pub currency: Option<String>,
}

impl From<GetUSTransactionHistoryOptions> for longbridge::trade::GetUSTransactionHistoryOptions {
    #[inline]
    fn from(opts: GetUSTransactionHistoryOptions) -> Self {
        let mut opts2 = longbridge::trade::GetUSTransactionHistoryOptions::new(
            from_datetime(opts.start_at),
            from_datetime(opts.end_at),
        );
        if let Some(page) = opts.page {
            opts2.page = page;
        }
        if let Some(size) = opts.size {
            opts2.size = size;
        }
        opts2.types = opts.types.unwrap_or_default();
        opts2.currency = opts.currency.unwrap_or_default();
        opts2
    }
}
