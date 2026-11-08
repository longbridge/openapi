//! Strategy signals and the catalyst facts behind them

mod context;
/// Signal and fact types
pub mod types;

pub use context::SignalContext;
pub use types::{
    AnomalyDetection, AnomalyThresholds, FactDataSource, FactDirection, FactFactor, FactNlInfo,
    FactSymbol, FactType, NlTag, Outlook, SecurityFact, SecurityFactsOptions, Signal, SignalStatus,
    SignalsOptions, SignalsResponse,
};
