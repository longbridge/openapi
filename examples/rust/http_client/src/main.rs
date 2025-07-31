use longport::httpclient::HttpClient;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let http_cli = HttpClient::from_env()?;
    let resp = http_cli
        .request("GET".parse()?, "/v1/trade/execution/today")
        .response::<String>()
        .send()
        .await?;
    println!("{resp}");
    Ok(())
}
