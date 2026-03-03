const { Config, QuoteContext, SubType, OAuth, OAuthToken } = require("longport");

let globalCtx;

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
  let config = Config.fromOAuth(token);
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnQuote((_, event) => console.log(event.toString()));
  globalCtx.subscribe(["TSLA.US"], [SubType.Quote], true);
}

Promise.all([main()]).catch((err) => console.error(err));
