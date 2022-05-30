use std::sync::Arc;

use longbridge::{quote::SubFlags, Config, QuoteContext, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for quote APIs
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;

    // Subscribe
    ctx.subscribe(["700.HK"], SubFlags::QUOTE, true).await?;

    // Receive push events
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }

    Ok(())
}
