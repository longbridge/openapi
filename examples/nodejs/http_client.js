const { HttpClient, OAuth } = require("longport");

async function main() {
  const clientId = "your-client-id";
  const oauth = new OAuth(clientId);
  const token = await oauth.authorize((url) => {
    console.log(url);
  });
  let cli = HttpClient.fromOauth(clientId, token.accessToken);
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
