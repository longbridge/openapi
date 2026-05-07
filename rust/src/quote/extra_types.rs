#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Domain A – Fundamental Data helpers
// ---------------------------------------------------------------------------

/// Optional parameters for `financial_report`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct FinancialReportOptions {
    /// Report kind (e.g. "annual", "interim").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Report type (e.g. "income", "balance", "cashflow").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
}

/// Optional parameters for `institution_rating_detail`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct InstitutionRatingDetailOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Optional parameters for `dividends`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DividendsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Optional parameters for `valuation`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ValuationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Optional parameters for `valuation_history`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ValuationHistoryOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Optional parameters for `corporate_actions`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CorporateActionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

/// Optional parameters for `operating_data`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct OperatingDataOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

// ---------------------------------------------------------------------------
// Domain B – Market Data helpers
// ---------------------------------------------------------------------------

/// Optional parameters for `broker_holding`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BrokerHoldingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// Optional parameters for `ah_premium_klines`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct AhPremiumKlinesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

// ---------------------------------------------------------------------------
// Domain C – Calendar helper
// ---------------------------------------------------------------------------

/// Optional parameters for `finance_calendar`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinanceCalendarOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
