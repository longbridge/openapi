const { Config, TradeContext } = require("longport");

let config = Config.fromEnv();
TradeContext.new(config)
  .then((ctx) => ctx.todayOrders())
  .then((resp) => {
    for (let obj of resp) {
      console.log(obj.toString());
    }
  });
