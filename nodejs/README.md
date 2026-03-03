# LongPort OpenAPI SDK for Node.js

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/nodejs/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

## Examples

Runnable examples live in `examples/nodejs/`:

- `examples/nodejs/account_asset.js`
- `examples/nodejs/http_client.js`
- `examples/nodejs/subscribe_candlesticks.js`
- `examples/nodejs/subscribe_quote.js`
- `examples/nodejs/submit_order.js`
- `examples/nodejs/today_orders.js`

## Quickstart

_Install LongPort OpenAPI SDK_

```bash
npm install longport
```

### Authentication

LongPort OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Authorize and Get Token**

```javascript
const { Config, OAuth, OAuthToken } = require("longport");

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
  const config = Config.fromOAuth(token);
  // Use config to create contexts...
}

main();
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```bash
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```javascript
const { Config, QuoteContext, OAuth, OAuthToken } = require("longport");

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
  const config = Config.fromOAuth(token);
  const ctx = await QuoteContext.new(config);
  const resp = await ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"]);
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

main();
```

## Quote API _(Subscribe quotes)_

```javascript
const { Config, QuoteContext, SubType, OAuth, OAuthToken } = require("longport");

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
  const config = Config.fromOAuth(token);
  const ctx = await QuoteContext.new(config);
  ctx.setOnQuote((_, event) => console.log(event.toString()));
  await ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
    [SubType.Quote],
    true
  );
}

main();
```

## Trade API _(Submit order)_

```javascript
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
  const config = Config.fromOAuth(token);
  const ctx = await TradeContext.new(config);
  const resp = await ctx.submitOrder({
    symbol: "700.HK",
    orderType: OrderType.LO,
    side: OrderSide.Buy,
    timeInForce: TimeInForceType.Day,
    submittedPrice: new Decimal("50"),
    submittedQuantity: 200,
  });
  console.log(resp.toString());
}

main();
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If the script exits, you won't receive push events; keep Node running.
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
