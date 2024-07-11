use std::sync::Arc;

use longport::{
    quote::{Period, QuoteContext},
    Config, PushCandlestickMode,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config =
        Arc::new(Config::from_env()?.push_candlestick_mode(PushCandlestickMode::Confirmed));
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    println!("member id: {}", ctx.member_id());
    ctx.subscribe_candlesticks("600000.SH", Period::FiveMinute)
        .await?;

    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }
    Ok(())
}
