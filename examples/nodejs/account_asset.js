const { Config, TradeContext } = require("longbridge");

let config = Config.fromEnv();
TradeContext.new(config)
  .then((ctx) => ctx.accountBalance())
  .then((resp) => {
    for (let obj of resp) {
      console.log(obj.toString());
    }
  });
