use std::sync::Arc;

use longbridge::{oauth::OAuthBuilder, trade::TradeContext, Config};
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

    let resp = ctx.today_orders(None).await?;
    for obj in resp {
        println!("{obj:?}");
    }
    Ok(())
}
