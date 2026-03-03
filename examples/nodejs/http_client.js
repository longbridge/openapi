const { HttpClient, OAuth, OAuthToken } = require("longport");

async function getToken() {
  const clientId = "your-client-id";
  try {
    const token = OAuthToken.load();
    if (token.isExpired()) throw new Error("token expired");
    if (token.expiresSoon()) {
      const oauth = new OAuth(clientId);
      try {
        const newToken = await oauth.refresh(token);
        newToken.save();
        return newToken;
      } catch (_) {}
    } else {
      return token;
    }
  } catch (_) {}
  const oauth = new OAuth(clientId);
  const token = await oauth.authorize((url) => {
    console.log("Open this URL to authorize: " + url);
  });
  token.save();
  return token;
}

async function main() {
  const token = await getToken();
  let cli = HttpClient.fromOAuth(token);
  let resp = await cli.request("get", "/v1/trade/execution/today");
  console.log(resp);
}

Promise.all([main()]).catch((err) => console.error(err));
