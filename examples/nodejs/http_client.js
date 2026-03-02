const { HttpClient, OAuth } = require("longport");

async function main() {
  const oauth = new OAuth("your-client-id");
  const token = await oauth.authorize((url) => {
    console.log(url);
  });
  let cli = HttpClient.fromOauth(oauth.clientId, token.accessToken);
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
