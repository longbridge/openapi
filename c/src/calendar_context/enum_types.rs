use longbridge_c_macros::CEnum;

/// Financial calendar event category
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::calendar::types::CalendarCategory")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCalendarCategory {
    /// Earnings reports
    #[c(remote = "Report")]
    CalendarCategoryReport,
    /// Dividend announcements
    #[c(remote = "Dividend")]
    CalendarCategoryDividend,
    /// Stock splits
    #[c(remote = "Split")]
    CalendarCategorySplit,
    /// IPOs
    #[c(remote = "Ipo")]
    CalendarCategoryIpo,
    /// Macro-economic data
    #[c(remote = "MacroData")]
    CalendarCategoryMacroData,
    /// Market closure days
    #[c(remote = "Closed")]
    CalendarCategoryClosed,
    /// Shareholder / analyst meetings
    #[c(remote = "Meeting")]
    CalendarCategoryMeeting,
    /// Stock consolidations / mergers
    #[c(remote = "Merge")]
    CalendarCategoryMerge,
}
