use std::sync::Arc;

use longport::{
    oauth::{OAuth, OAuthToken},
    trade::TradeContext,
    Config,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let token = match OAuthToken::load() {
        Ok(token) => token,
        Err(_) => {
            let oauth = OAuth::new("your-client-id");
            let token = oauth
                .authorize(|url| println!("Open this URL to authorize: {url}"))
                .await?;
            token.save()?;
            token
        }
    };

    let config = Arc::new(Config::from_oauth(&token));
    let (ctx, _) = TradeContext::try_new(config).await?;

    let resp = ctx.today_orders(None).await?;
    for obj in resp {
        println!("{obj:?}");
    }
    Ok(())
}
