const { Config, QuoteContext, SubType, OAuthBuilder } = require('longbridge');

let globalCtx;

async function main() {
  const oauth = await OAuthBuilder.build("your-client-id", (url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let config = Config.fromOAuth(oauth);
  globalCtx = await QuoteContext.new(config);
  globalCtx.setOnQuote((_, event) => console.log(event.toString()));
  globalCtx.subscribe(["TSLA.US"], [SubType.Quote], true);
}

Promise.all([main()]).catch((err) => console.error(err));
