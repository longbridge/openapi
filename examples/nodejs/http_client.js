const { HttpClient, OAuthBuilder } = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build("your-client-id", (url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let cli = HttpClient.fromOAuth(oauth);
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
