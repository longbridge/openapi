# Longbridge OpenAPI SDK for Node.js

`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).

## Documentation

- SDK docs: https://longbridge.github.io/openapi/nodejs/index.html
- Longbridge OpenAPI: https://open.longbridge.com/en/

## Examples

Runnable examples live in `examples/nodejs/`:

- `examples/nodejs/account_asset.js`
- `examples/nodejs/http_client.js`
- `examples/nodejs/subscribe_candlesticks.js`
- `examples/nodejs/subscribe_quote.js`
- `examples/nodejs/submit_order.js`
- `examples/nodejs/today_orders.js`

## Quickstart

_Install Longbridge OpenAPI SDK_

```bash
npm install longbridge
```

### Authentication

Longbridge OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

_bash / macOS / Linux_

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

_PowerShell (Windows)_

```powershell
Invoke-RestMethod -Method Post -Uri https://openapi.longbridgeapp.com/oauth2/register `
  -ContentType "application/json" `
  -Body '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "client_name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Build an OAuth client and create Config**

`OAuthBuilder.build()` loads a cached token from
`~/.longbridge-openapi/tokens/<client_id>`
(`%USERPROFILE%\.longbridge-openapi\tokens\<client_id>` on Windows) if one
exists and is still valid, or starts the browser authorization flow
automatically.  The token is persisted to the same path after a successful
authorization or refresh.

```javascript
const { OAuthBuilder, Config } = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build(
    "your-client-id",
    (url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
  // Use config to create contexts...
}

main();
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables (macOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables (Windows)_

```bash
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

Then create a config from the environment:

```javascript
const { Config } = require('longbridge');
const config = Config.fromApikeyEnv();
```

## Quote API _(Get basic information of securities)_

```javascript
const { OAuthBuilder, Config, QuoteContext } = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build(
    "your-client-id",
    (url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
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
const { OAuthBuilder, Config, QuoteContext, SubType } = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build(
    "your-client-id",
    (url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
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
  OAuthBuilder,
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
} = require('longbridge');

async function main() {
  const oauth = await OAuthBuilder.build(
    "your-client-id",
    (url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
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
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
