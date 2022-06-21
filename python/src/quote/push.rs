use longbridge::quote::{
    PushBrokers, PushCandlestick, PushDepth, PushEvent, PushEventDetail, PushQuote, PushTrades,
};
use pyo3::prelude::*;

use crate::quote::context::Callbacks;

pub(crate) fn handle_push_event(callbacks: &Callbacks, event: PushEvent) {
    match event.detail {
        PushEventDetail::Quote(quote) => handle_quote(callbacks, event.symbol, quote),
        PushEventDetail::Depth(depth) => handle_depth(callbacks, event.symbol, depth),
        PushEventDetail::Brokers(brokers) => handle_brokers(callbacks, event.symbol, brokers),
        PushEventDetail::Trade(trades) => handle_trades(callbacks, event.symbol, trades),
        PushEventDetail::Candlestick(candlestick) => {
            handle_candlesticks(callbacks, event.symbol, candlestick)
        }
    }
}

fn handle_quote(callbacks: &Callbacks, symbol: String, quote: PushQuote) {
    if let Some(callback) = &callbacks.quote {
        let _ = Python::with_gil(|py| {
            callback.call(
                py,
                (symbol, crate::quote::types::PushQuote::try_from(quote)?),
                None,
            )
        });
    }
}

fn handle_depth(callbacks: &Callbacks, symbol: String, depth: PushDepth) {
    if let Some(callback) = &callbacks.depth {
        let _ = Python::with_gil(|py| {
            callback.call(
                py,
                (symbol, crate::quote::types::PushDepth::try_from(depth)?),
                None,
            )
        });
    }
}

fn handle_brokers(callbacks: &Callbacks, symbol: String, brokers: PushBrokers) {
    if let Some(callback) = &callbacks.brokers {
        let _ = Python::with_gil(|py| {
            callback.call(
                py,
                (symbol, crate::quote::types::PushBrokers::try_from(brokers)?),
                None,
            )
        });
    }
}

fn handle_trades(callbacks: &Callbacks, symbol: String, trades: PushTrades) {
    if let Some(callback) = &callbacks.trades {
        let _ = Python::with_gil(|py| {
            callback.call(
                py,
                (symbol, crate::quote::types::PushTrades::try_from(trades)?),
                None,
            )
        });
    }
}

fn handle_candlesticks(callbacks: &Callbacks, symbol: String, candlestick: PushCandlestick) {
    if let Some(callback) = &callbacks.trades {
        let _ = Python::with_gil(|py| {
            callback.call(
                py,
                (
                    symbol,
                    crate::quote::types::PushCandlestick::try_from(candlestick)?,
                ),
                None,
            )
        });
    }
}
