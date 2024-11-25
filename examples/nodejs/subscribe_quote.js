const { Config, QuoteContext, SubType } = require("longport");

let globalCtx;

async function main() {
  let config = Config.fromEnv();
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnQuote((_, event) => console.log(event.toString()));
  globalCtx.subscribe(["TSLA.US"], [SubType.Quote], true);
}

Promise.all([main()]).catch((err) => console.error(err));
