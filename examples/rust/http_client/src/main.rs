use longbridge::{
    httpclient::{HttpClient, HttpClientConfig},
    oauth::OAuthBuilder,
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
    let http_cli = HttpClient::new(HttpClientConfig::from_oauth(oauth));
    let resp = http_cli
        .request("GET".parse()?, "/v1/trade/execution/today")
        .response::<String>()
        .send()
        .await?;
    println!("{resp}");
    Ok(())
}
