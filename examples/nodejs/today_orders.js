const { Config, TradeContext, OAuth } = require("longport");

async function main() {
  const oauth = new OAuth("your-client-id");
  const token = await oauth.authorize((url) => {
    console.log(url);
  });
  let config = Config.fromOauth("your-client-id", token.accessToken);
  let ctx = await TradeContext.new(config);
  let resp = await ctx.todayOrders();
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

Promise.all([main()]).catch((err) => console.error(err));
