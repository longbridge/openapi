const { Config, QuoteContext, Period, TradeSessions, OAuth } = require("longport");

let globalCtx;

async function main() {
  const oauth = new OAuth("your-client-id");
  const token = await oauth.authorize((url) => {
    console.log(url);
  });
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
