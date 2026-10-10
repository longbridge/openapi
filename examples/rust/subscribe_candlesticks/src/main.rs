use std::sync::Arc;

use longbridge::{
    oauth::OAuthBuilder,
    quote::{Period, QuoteContext, TradeSessions},
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
    println!("member id: {}", ctx.member_id().await?);
    ctx.subscribe_candlesticks(".SPX.US", Period::OneMinute, TradeSessions::All)
        .await?;

    while let Some(event) = receiver.recv().await {
        println!("{event:?}");
    }
    Ok(())
}
