use longbridge::quote::{
    PushBrokers, PushCandlestick, PushDepth, PushEvent, PushEventDetail, PushQuote, PushTrades,
};
use pyo3::prelude::*;

use crate::{async_callback, quote::context::Callbacks};

pub(crate) fn handle_push_event(
    callbacks: &Callbacks,
    event: PushEvent,
    event_loop: Option<&Bound<PyAny>>,
) {
    match event.detail {
        PushEventDetail::Quote(quote) => handle_quote(callbacks, event.symbol, quote, event_loop),
        PushEventDetail::Depth(depth) => handle_depth(callbacks, event.symbol, depth, event_loop),
        PushEventDetail::Brokers(brokers) => {
            handle_brokers(callbacks, event.symbol, brokers, event_loop)
        }
        PushEventDetail::Trade(trades) => {
            handle_trades(callbacks, event.symbol, trades, event_loop)
        }
        PushEventDetail::Candlestick(candlestick) => {
            handle_candlesticks(callbacks, event.symbol, candlestick, event_loop)
        }
    }
}

fn handle_quote(
    callbacks: &Callbacks,
    symbol: String,
    quote: PushQuote,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.quote {
        let _ = Python::attach(|py| {
            let args = (symbol, crate::quote::types::PushQuote::try_from(quote)?);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

fn handle_depth(
    callbacks: &Callbacks,
    symbol: String,
    depth: PushDepth,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.depth {
        let _ = Python::attach(|py| {
            let args = (symbol, crate::quote::types::PushDepth::try_from(depth)?);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

fn handle_brokers(
    callbacks: &Callbacks,
    symbol: String,
    brokers: PushBrokers,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.brokers {
        let _ = Python::attach(|py| {
            let args = (symbol, crate::quote::types::PushBrokers::try_from(brokers)?);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

fn handle_trades(
    callbacks: &Callbacks,
    symbol: String,
    trades: PushTrades,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.trades {
        let _ = Python::attach(|py| {
            let args = (symbol, crate::quote::types::PushTrades::try_from(trades)?);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

fn handle_candlesticks(
    callbacks: &Callbacks,
    symbol: String,
    candlestick: PushCandlestick,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.candlestick {
        let _ = Python::attach(|py| {
            let args = (
                symbol,
                crate::quote::types::PushCandlestick::try_from(candlestick)?,
            );
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}
