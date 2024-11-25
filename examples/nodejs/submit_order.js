const {
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
} = require("longport");

async function main() {
  let config = Config.fromEnv();
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
