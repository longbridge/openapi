const { Config, QuoteContext, SubType, OAuth } = require("longport");

let globalCtx;

async function main() {
  const oauth = new OAuth("your-client-id");
  const token = await oauth.authorize((url) => {
    console.log(url);
  });
  let config = Config.fromOauth("your-client-id", token.accessToken);
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnQuote((_, event) => console.log(event.toString()));
  globalCtx.subscribe(["TSLA.US"], [SubType.Quote], true);
}

Promise.all([main()]).catch((err) => console.error(err));
