const { Config, QuoteContext, Period } = require("longport");

let globalCtx;

async function main() {
  let config = Config.fromEnv();
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnCandlestick((_, event) => console.log(event.toString()));
  globalCtx.subscribeCandlesticks("AAPL.US", Period.Min_1);
}

Promise.all([main()]).catch((err) => console.error(err));
