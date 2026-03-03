use longport::{
    httpclient::{HttpClient, HttpClientConfig},
    oauth::{OAuth, OAuthToken},
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

    let http_cli = HttpClient::new(HttpClientConfig::from_oauth(&token));
    let resp = http_cli
        .request("GET".parse()?, "/v1/trade/execution/today")
        .response::<String>()
        .send()
        .await?;
    println!("{resp}");
    Ok(())
}
