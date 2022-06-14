const {
  Config,
  TradeContext,
  SubmitOrderOptions,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
} = require("longbridge");

let config = Config.fromEnv();
TradeContext.new(config)
  .then((ctx) =>
    ctx.submitOrder(
      new SubmitOrderOptions(
        "700.HK",
        OrderType.LO,
        OrderSide.Buy,
        "200",
        TimeInForceType.Day
      ).submittedPrice(new Decimal("50"))
    )
  )
  .then((resp) => console.log(resp.toString()));
