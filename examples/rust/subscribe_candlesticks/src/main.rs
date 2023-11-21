use std::sync::Arc;

use longport::{
    quote::{Period, QuoteContext},
    Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::from_env()?);
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute)
        .await?;
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }
    Ok(())
}
