use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::pyclass;

use crate::decimal::PyDecimal;

/// Forex order state.
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::forex::ForexOrderStatus")]
pub(crate) enum ForexOrderStatus {
    /// Processing (includes manual review); keep polling.
    Processing,
    /// Conversion succeeded.
    Success,
    /// Conversion failed; frozen funds have been returned.
    Failed,
}

/// A forex quote.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::forex::ForexQuote")]
pub(crate) struct ForexQuote {
    /// Quote id, used when submitting the order
    quote_id: String,
    /// Customer execution rate (standard currency-pair terms)
    rate: PyDecimal,
    /// Quote expiry, Unix milliseconds
    expire_at: i64,
    /// The standard currency pair for `rate`, format `BASE/QUOTE`
    ccy_pair: String,
}

/// A forex order's state.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::forex::ForexOrderDetail")]
pub(crate) struct ForexOrderDetail {
    /// Order state
    state: ForexOrderStatus,
    /// Execution rate (standard currency-pair terms); `None` until filled
    #[py(opt)]
    rate: Option<PyDecimal>,
    /// Converted-from amount (actual value once filled); `None` until filled
    #[py(opt)]
    from_amount: Option<PyDecimal>,
    /// Converted-to amount (actual value once filled); `None` until filled
    #[py(opt)]
    to_amount: Option<PyDecimal>,
    /// Failure reason; empty when not failed
    fail_reason: String,
}
