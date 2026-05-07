//! Dollar-Cost Averaging (DCA) plan module.

mod context;
mod types;

pub use context::DcaContext;
pub use types::{
    CheckDcaSupportOptions, CreateDcaPlanOptions, DcaHistoryOptions, UpdateDcaPlanOptions,
};
