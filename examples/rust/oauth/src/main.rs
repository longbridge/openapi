use std::sync::Arc;

use longport::{oauth::OAuth, Config};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let oauth = OAuth::new("your-client-id");
    let token = oauth
        .authorize(|url| {
            println!("Open this URL to authorize: {url}");
        })
        .await?;

    println!("Access token: {}", token.access_token);
    println!("Expires at: {}", token.expires_at);

    let config = Arc::new(Config::from_oauth(oauth.client_id(), &token.access_token));
    println!("Config created: {config:?}");

    Ok(())
}
