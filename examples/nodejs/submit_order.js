const {
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
} = require("longport");

let config = Config.fromEnv();
TradeContext.new(config)
  .then((ctx) =>
    ctx.submitOrder({
      symbol: "700.HK",
      orderType: OrderType.LO,
      side: OrderSide.Buy,
      timeInForce: TimeInForceType.Day,
      submittedPrice: new Decimal("50"),
      submittedQuantity: 200,
    })
  )
  .then((resp) => console.log(resp.toString()));
