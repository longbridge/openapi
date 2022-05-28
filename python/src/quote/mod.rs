mod context;
mod push;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &PyModule) -> PyResult<()> {
    parent.add_class::<types::DerivativeType>()?;
    parent.add_class::<types::TradeStatus>()?;
    parent.add_class::<types::TradeSession>()?;
    parent.add_class::<types::SubType>()?;
    parent.add_class::<types::TradeDirection>()?;
    parent.add_class::<types::OptionType>()?;
    parent.add_class::<types::Period>()?;
    parent.add_class::<types::AdjustType>()?;
    parent.add_class::<types::SecurityStaticInfo>()?;
    parent.add_class::<types::PrePostQuote>()?;
    parent.add_class::<types::SecurityQuote>()?;
    parent.add_class::<types::OptionQuote>()?;
    parent.add_class::<types::WarrantQuote>()?;
    parent.add_class::<types::Depth>()?;
    parent.add_class::<types::SecurityDepth>()?;
    parent.add_class::<types::Brokers>()?;
    parent.add_class::<types::SecurityBrokers>()?;
    parent.add_class::<types::ParticipantInfo>()?;
    parent.add_class::<types::Trade>()?;
    parent.add_class::<types::IntradayLine>()?;
    parent.add_class::<types::Candlestick>()?;
    parent.add_class::<types::StrikePriceInfo>()?;
    parent.add_class::<types::IssuerInfo>()?;
    parent.add_class::<types::TradingSessionInfo>()?;
    parent.add_class::<types::MarketTradingSession>()?;
    parent.add_class::<types::RealtimeQuote>()?;
    parent.add_class::<types::PushQuote>()?;
    parent.add_class::<types::PushDepth>()?;
    parent.add_class::<types::PushBrokers>()?;
    parent.add_class::<types::PushTrades>()?;

    parent.add_class::<context::QuoteContext>()?;
    Ok(())
}
