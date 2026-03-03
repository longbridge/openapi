use std::sync::Arc;

use longport::{
    oauth::{OAuth, OAuthToken},
    trade::TradeContext,
    Config,
};
use tracing_subscriber::EnvFilter;

async fn get_token() -> Result<OAuthToken, Box<dyn std::error::Error>> {
    let client_id = "your-client-id";
    let token = match OAuthToken::load() {
        Ok(token) if token.is_expired() => {
            let oauth = OAuth::new(client_id);
            let token = oauth
                .authorize(|url| println!("Open this URL to authorize: {url}"))
                .await?;
            token.save()?;
            token
        }
        Ok(token) if token.expires_soon() => {
            let oauth = OAuth::new(client_id);
            match oauth.refresh(&token).await {
                Ok(new_token) => {
                    new_token.save()?;
                    new_token
                }
                Err(_) => {
                    let token = oauth
                        .authorize(|url| println!("Open this URL to authorize: {url}"))
                        .await?;
                    token.save()?;
                    token
                }
            }
        }
        Ok(token) => token,
        Err(_) => {
            let oauth = OAuth::new(client_id);
            let token = oauth
                .authorize(|url| println!("Open this URL to authorize: {url}"))
                .await?;
            token.save()?;
            token
        }
    };
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let token = get_token().await?;
    let config = Arc::new(Config::from_oauth(&token));
    let (ctx, _) = TradeContext::try_new(config).await?;

    let resp = ctx.today_orders(None).await?;
    for obj in resp {
        println!("{obj:?}");
    }
    Ok(())
}
