//! Request option builders for the forex endpoints.

use rust_decimal::Decimal;
use serde::Serialize;

/// Optional amount parameters for [`ForexContext::quote`]. Provide at most one
/// of `amount` / `target_amount`; the server validates the choice.
///
/// [`ForexContext::quote`]: crate::forex::ForexContext::quote
#[derive(Debug, Default, Clone, Serialize)]
pub struct GetForexQuoteOptions {
    /// Convert-out amount (ISO currency `from`), min 0.01, at most 2 decimals.
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Decimal>,
    /// Convert-in amount (ISO currency `to`).
    #[serde(skip_serializing_if = "Option::is_none")]
    target_amount: Option<Decimal>,
}

impl GetForexQuoteOptions {
    /// Create a new [`GetForexQuoteOptions`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the convert-out amount.
    #[inline]
    #[must_use]
    pub fn amount(mut self, amount: impl Into<Decimal>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Set the convert-in amount.
    #[inline]
    #[must_use]
    pub fn target_amount(mut self, target_amount: impl Into<Decimal>) -> Self {
        self.target_amount = Some(target_amount.into());
        self
    }
}

/// Parameters for [`ForexContext::submit_order`].
///
/// [`ForexContext::submit_order`]: crate::forex::ForexContext::submit_order
#[derive(Debug, Clone, Serialize)]
pub struct SubmitForexOrderOptions {
    quote_id: String,
    client_order_id: String,
}

impl SubmitForexOrderOptions {
    /// Create new submit options from a `quote_id` and a caller-supplied
    /// `client_order_id` (unique across all of the caller's accounts).
    #[inline]
    #[must_use]
    pub fn new(quote_id: impl Into<String>, client_order_id: impl Into<String>) -> Self {
        Self {
            quote_id: quote_id.into(),
            client_order_id: client_order_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_options_skip_none() {
        let s = serde_json::to_string(&GetForexQuoteOptions::new().amount(Decimal::new(100000, 2)))
            .unwrap();
        assert_eq!(s, r#"{"amount":"1000.00"}"#);
    }

    #[test]
    fn submit_options_serialize() {
        let s = serde_json::to_string(&SubmitForexOrderOptions::new("Q1", "c-1")).unwrap();
        assert_eq!(s, r#"{"quote_id":"Q1","client_order_id":"c-1"}"#);
    }
}
