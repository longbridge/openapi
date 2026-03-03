const { Config, TradeContext, OAuth, OAuthToken } = require("longport");

async function main() {
  let token;
  try {
    token = OAuthToken.load();
  } catch (_) {
    const oauth = new OAuth("your-client-id");
    token = await oauth.authorize((url) => {
      console.log("Open this URL to authorize: " + url);
    });
    token.save();
  }
  let config = Config.fromOAuth(token);
  let ctx = await TradeContext.new(config);
  let resp = await ctx.accountBalance();
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

Promise.all([main()]).catch((err) => console.error(err));
