const { Config, QuoteContext, Period } = require("longbridge");

let config = Config.fromEnv();
QuoteContext.new(config).then((ctx) => {
  ctx.setOnCandlestick((_, event) => console.log(event.toString()));
  ctx.subscribeCandlesticks("AAPL.US", Period.Min_1);
});
