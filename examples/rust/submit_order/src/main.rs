use std::sync::Arc;

use longport::{
    decimal,
    trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType, TradeContext},
    Config,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Arc::new(Config::from_env()?);
    let (ctx, _) = TradeContext::try_new(config).await?;

    let opts = SubmitOrderOptions::new(
        "700.HK",
        OrderType::LO,
        OrderSide::Buy,
        decimal!(200),
        TimeInForceType::Day,
    )
    .submitted_price(decimal!(50i32));
    let resp = ctx.submit_order(opts).await?;
    println!("{resp:?}");
    Ok(())
}
