use std::sync::Arc;

use longbridge::{
    oauth::OAuthBuilder,
    quote::{QuoteContext, SubFlags},
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
    let (ctx, mut receiver) = QuoteContext::new(config);
    ctx.subscribe(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"], SubFlags::QUOTE)
        .await?;
    while let Some(event) = receiver.recv().await {
        println!("{event:?}");
    }
    Ok(())
}
