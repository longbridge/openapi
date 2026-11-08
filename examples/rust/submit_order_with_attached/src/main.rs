use std::sync::Arc;

use longbridge::{
    decimal,
    oauth::OAuthBuilder,
    trade::{
        AttachedOrderType, OrderSide, OrderType, SubmitAttachedParams, SubmitOrderOptions,
        TimeInForceType, TradeContext,
    },
    Config,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let oauth = OAuthBuilder::new("your-client-id")
        .build(|url| println!("Open this URL to authorize: {url}"))
        .await?;
    let config = Arc::new(Config::from_oauth(oauth));
    let (ctx, _) = TradeContext::new(config);

    let attached = SubmitAttachedParams::new(AttachedOrderType::Bracket)
        .profit_taker_price(decimal!(220i32))
        .stop_loss_price(decimal!(180i32));

    let opts = SubmitOrderOptions::new(
        "AAPL.US",
        OrderType::LO,
        OrderSide::Buy,
        decimal!(1i32),
        TimeInForceType::Day,
    )
    .submitted_price(decimal!(200i32))
    .attached_params(attached);

    let resp = ctx.submit_order(opts).await?;
    println!("{resp:?}");
    Ok(())
}
