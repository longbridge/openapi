const { Config, QuoteContext, SubType, OAuth, OAuthToken } = require("longport");

let globalCtx;

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
  let config = Config.fromOAuth(token);
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnQuote((_, event) => console.log(event.toString()));
  globalCtx.subscribe(["TSLA.US"], [SubType.Quote], true);
}

Promise.all([main()]).catch((err) => console.error(err));
