use longbridge_c_macros::CEnum;

/// Institutional analyst recommendation
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::fundamental::types::InstitutionRecommend")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CInstitutionRecommend {
    /// Unknown
    #[c(remote = "Unknown")]
    InstitutionRecommendUnknown,
    /// Strong buy
    #[c(remote = "StrongBuy")]
    InstitutionRecommendStrongBuy,
    /// Buy
    #[c(remote = "Buy")]
    InstitutionRecommendBuy,
    /// Hold
    #[c(remote = "Hold")]
    InstitutionRecommendHold,
    /// Sell
    #[c(remote = "Sell")]
    InstitutionRecommendSell,
    /// Strong sell
    #[c(remote = "StrongSell")]
    InstitutionRecommendStrongSell,
    /// Underperform
    #[c(remote = "Underperform")]
    InstitutionRecommendUnderperform,
    /// No opinion
    #[c(remote = "NoOpinion")]
    InstitutionRecommendNoOpinion,
}

/// Financial report kind
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::fundamental::types::FinancialReportKind")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CFinancialReportKind {
    /// Income statement
    #[c(remote = "IncomeStatement")]
    FinancialReportKindIncomeStatement,
    /// Balance sheet
    #[c(remote = "BalanceSheet")]
    FinancialReportKindBalanceSheet,
    /// Cash flow statement
    #[c(remote = "CashFlow")]
    FinancialReportKindCashFlow,
    /// All statements (default)
    #[c(remote = "All")]
    FinancialReportKindAll,
}

/// Financial report period
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::fundamental::types::FinancialReportPeriod")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CFinancialReportPeriod {
    /// Annual report
    #[c(remote = "Annual")]
    FinancialReportPeriodAnnual,
    /// Semi-annual report
    #[c(remote = "SemiAnnual")]
    FinancialReportPeriodSemiAnnual,
    /// Q1 report
    #[c(remote = "Q1")]
    FinancialReportPeriodQ1,
    /// Q2 report
    #[c(remote = "Q2")]
    FinancialReportPeriodQ2,
    /// Q3 report
    #[c(remote = "Q3")]
    FinancialReportPeriodQ3,
    /// Full quarterly report
    #[c(remote = "QuarterlyFull")]
    FinancialReportPeriodQuarterlyFull,
    /// Three-quarter report (first three quarters)
    #[c(remote = "ThreeQ")]
    FinancialReportPeriodThreeQ,
}

/// ETF asset allocation element type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::fundamental::types::ElementType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CElementType {
    /// Unknown
    #[c(remote = "Unknown")]
    ElementTypeUnknown,
    /// Holdings
    #[c(remote = "Holdings")]
    ElementTypeHoldings,
    /// Regional
    #[c(remote = "Regional")]
    ElementTypeRegional,
    /// Asset class
    #[c(remote = "AssetClass")]
    ElementTypeAssetClass,
    /// Industry
    #[c(remote = "Industry")]
    ElementTypeIndustry,
}
