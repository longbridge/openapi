const { Config, QuoteContext, Period, TradeSessions, OAuth } = require('longbridge');

let globalCtx;

async function main() {
  const oauth = await OAuth.build("your-client-id", (url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let config = Config.fromOAuth(oauth);
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnCandlestick((_, event) => console.log(event.toString()));
  await globalCtx.subscribeCandlesticks(
    "AAPL.US",
    Period.Min_1,
    TradeSessions.Intraday
  );
}

Promise.all([main()]).catch((err) => console.error(err));
