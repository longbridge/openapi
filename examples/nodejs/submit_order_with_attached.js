const {
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
  AttachedOrderType,
  OAuth,
} = require('longbridge');

async function main() {
  const oauth = await OAuth.build("your-client-id", (_, url) => {
    console.log("Open this URL to authorize: " + url);
  });
  let config = Config.fromOAuth(oauth);
  let ctx = TradeContext.new(config);

  let resp = await ctx.submitOrder({
    symbol: "AAPL.US",
    orderType: OrderType.LO,
    side: OrderSide.Buy,
    submittedQuantity: new Decimal(1),
    timeInForce: TimeInForceType.Day,
    submittedPrice: new Decimal(200),
    attachedParams: {
      attachedOrderType: AttachedOrderType.Bracket,
      profitTakerPrice: new Decimal(220),
      stopLossPrice: new Decimal(180),
    },
  });
  console.log(resp.toString());
}

Promise.all([main()]).catch((err) => console.error(err));
