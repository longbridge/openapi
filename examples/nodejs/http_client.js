const { HttpClient, OAuth, OAuthToken } = require("longport");

async function main() {
  let token;
  try {
    token = OAuthToken.load();
  } catch (_) {
    const oauth = new OAuth("your-client-id");
    token = await oauth.authorize((url) => {
      console.log("Open this URL to authorize: " + url);
    });
    token.save();
  }
  let cli = HttpClient.fromOAuth(token);
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
