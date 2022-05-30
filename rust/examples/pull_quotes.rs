use std::sync::Arc;

use longbridge::{Config, QuoteContext, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for quote APIs
    let (ctx, _) = QuoteContext::try_new(config).await?;

    // Get basic information of securities
    let resp = ctx
        .quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
        .await?;
    println!("{:?}", resp);

    Ok(())
}
