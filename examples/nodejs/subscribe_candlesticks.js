const { Config, QuoteContext, Period, TradeSessions } = require("longport");

let globalCtx;

async function main() {
  let config = Config.fromEnv();
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnCandlestick((_, event) => console.log(event.toString()));
  globalCtx.subscribeCandlesticks(
    "AAPL.US",
    Period.Min_1,
    TradeSessions.Normal
  );
}

Promise.all([main()]).catch((err) => console.error(err));
