use longport::httpclient::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_cli = HttpClient::from_env()?;
    let resp = http_cli
        .request("GET".parse()?, "/v1/trade/execution/today")
        .response::<String>()
        .send()
        .await?;
    println!("{}", resp);
    Ok(())
}
