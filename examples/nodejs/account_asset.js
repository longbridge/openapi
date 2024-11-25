const { Config, TradeContext } = require("longport");

async function main() {
  let config = Config.fromEnv();
  let ctx = await TradeContext.new(config);
  let resp = await ctx.accountBalance();
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

Promise.all([main()]).catch((err) => console.error(err));
