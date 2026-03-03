const { Config, QuoteContext, Period, TradeSessions, OAuth, OAuthToken } = require("longport");

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
  globalCtx.setOnCandlestick((_, event) => console.log(event.toString()));
  globalCtx.subscribeCandlesticks(
    "AAPL.US",
    Period.Min_1,
    TradeSessions.Intraday
  );
}

Promise.all([main()]).catch((err) => console.error(err));
