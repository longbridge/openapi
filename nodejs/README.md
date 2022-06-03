# Longbridge OpenAPI SDK for Node.js

`longbridge` provides an easy-to-use interface for invokes [`Longbridge OpenAPI`](https://open.longbridgeapp.com/en/).

## Quickstart

_Install Longbridge OpenAPI SDK_

```bash
npm install longbridge
```

_Setting environment variables(MacOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```javascript
import { Config, QuoteContext } from 'longbridge'

let config = Config.fromEnv()
let ctx = new QuoteContext(config)
await ctx.open()
let resp = await ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
for (let obj of resp) {
    console.log(obj.toString())
}
```

## Quote API _(Subscribe quotes)_

```javascript
import { Config, QuoteContext, SubType } from 'longbridge'

let config = Config.fromEnv()
let ctx = new QuoteContext(config, (_, event) => console.log(event.toString()))
await ctx.open()
await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote], true)
```

## Trade API _(Submit order)_

```javascript
import { Config, TradeContext, SubmitOrderOptions, OrderType, OrderSide, Decimal, TimeInForceType } from 'longbridge'

let config = Config.fromEnv()
let ctx = new TradeContext(config)
await ctx.open()
let resp = await ctx.submitOrder(
    new SubmitOrderOptions("700.HK", OrderType.LO, OrderSide.Buy, new Decimal("200"), TimeInForceType.Day)
        .price(new Decimal("300"))
)
console.log(resp)
```

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
