use std::sync::Arc;

use longport::{
    oauth::OAuth,
    quote::{QuoteContext, SubFlags},
    Config,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let oauth = OAuth::new("your-client-id");
    let token = oauth
        .authorize(|url| println!("Open this URL to authorize: {url}"))
        .await?;

    let config = Arc::new(Config::from_oauth(oauth.client_id(), &token.access_token));
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;
    ctx.subscribe(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"], SubFlags::QUOTE)
        .await?;
    while let Some(event) = receiver.recv().await {
        println!("{event:?}");
    }
    Ok(())
}
