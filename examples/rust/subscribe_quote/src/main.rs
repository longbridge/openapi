use std::sync::Arc;

use longport::{
    quote::{QuoteContext, SubFlags},
    Config,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::from_env()?);
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ctx.subscribe(
        ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
        SubFlags::QUOTE,
        true,
    )
    .await?;
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }
    Ok(())
}
