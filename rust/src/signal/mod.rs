//! Strategy signals and the catalyst facts behind them

mod context;
/// Signal and fact types
pub mod types;

pub use context::SignalContext;
pub use types::{Outlook, SecurityFactsOptions, Signal, SignalsOptions, SignalsResponse};
