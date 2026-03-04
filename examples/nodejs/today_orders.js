const { Config, TradeContext, OAuthBuilder } = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build("your-client-id", (url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let config = Config.fromOAuth(oauth);
  let ctx = await TradeContext.new(config);
  let resp = await ctx.todayOrders();
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

Promise.all([main()]).catch((err) => console.error(err));
