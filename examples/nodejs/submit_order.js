const {
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
  OAuth,
  OAuthToken,
} = require("longport");

async function getToken() {
  const clientId = "your-client-id";
  try {
    const token = OAuthToken.load();
    if (token.isExpired()) throw new Error("token expired");
    if (token.expiresSoon()) {
      const oauth = new OAuth(clientId);
      try {
        const newToken = await oauth.refresh(token);
        newToken.save();
        return newToken;
      } catch (_) {}
    } else {
      return token;
    }
  } catch (_) {}
  const oauth = new OAuth(clientId);
  const token = await oauth.authorize((url) => {
    console.log("Open this URL to authorize: " + url);
  });
  token.save();
  return token;
}

async function main() {
  const token = await getToken();
  let config = Config.fromOAuth(token);
  let ctx = await TradeContext.new(config);
  let resp = await ctx.submitOrder({
    symbol: "700.HK",
    orderType: OrderType.LO,
    side: OrderSide.Buy,
    timeInForce: TimeInForceType.Day,
    submittedPrice: new Decimal(50),
    submittedQuantity: new Decimal(200),
  });
  console.log(resp.toString());
}

Promise.all([main()]).catch((err) => console.error(err));
