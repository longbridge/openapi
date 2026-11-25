# Longbridge OpenAPI SDK for Node.js

`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).


## Context Types

| Context | Description |
|---------|-------------|
| `QuoteContext` | Real-time quotes, candlesticks, options, warrants, watchlists, push subscriptions |
| `TradeContext` | Orders, positions, account balance, executions, cash flow |
| `AssetContext` | Account statement download |
| `ContentContext` | News, community topics |
| `FundamentalContext` | Financial reports, analyst ratings, dividends, valuation, company overview, shareholders |
| `MarketContext` | Market status, broker holdings, A/H premium, trade statistics, anomaly alerts, index constituents |
| `CalendarContext` | Financial calendar (earnings, dividends, splits, IPOs, macro data, market closures) |
| `PortfolioContext` | Exchange rates, portfolio P&L analysis |
| `AlertContext` | Price alert management (add/enable/disable/delete) |
| `DCAContext` | Dollar-cost averaging plan management |
| `SharelistContext` | Community sharelist management |

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
curl -X POST https://openapi.longbridge.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

_PowerShell (Windows)_

```powershell
Invoke-RestMethod -Method Post -Uri https://openapi.longbridge.com/oauth2/register `
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

`OAuth.build()` loads a cached token from
`~/.longbridge/openapi/tokens/<client_id>`
(`%USERPROFILE%\.longbridge\openapi\tokens\<client_id>` on Windows) if one
exists and is still valid, or starts the browser authorization flow
automatically.  The token is persisted to the same path after a successful
authorization or refresh.

```javascript
const { OAuth, Config } = require('longbridge');

async function main() {
  const oauth = await OAuth.build(
    "your-client-id",
    (_, url) => console.log("Open this URL to authorize: " + url)
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

### Other environment variables

| Name                             | Description                                                                     |
|----------------------------------|---------------------------------------------------------------------------------|
| LONGBRIDGE_LANGUAGE              | Language identifier, `zh-CN`, `zh-HK` or `en` (Default: `en`)                   |
| LONGBRIDGE_HTTP_URL              | HTTP endpoint url (Default: `https://openapi.longbridge.com`)                   |
| LONGBRIDGE_QUOTE_WS_URL          | Quote websocket endpoint url (Default: `wss://openapi-quote.longbridge.com/v2`) |
| LONGBRIDGE_TRADE_WS_URL          | Trade websocket endpoint url (Default: `wss://openapi-trade.longbridge.com/v2`) |
| LONGBRIDGE_ENABLE_OVERNIGHT      | Enable overnight quote, `true` or `false` (Default: `false`)                    |
| LONGBRIDGE_PUSH_CANDLESTICK_MODE | `realtime` or `confirmed` (Default: `realtime`)                                 |
| LONGBRIDGE_QUOTE_TRANSPORT       | Transport for quote pull APIs, `ws` or `http` (Default: `ws`)                   |
| LONGBRIDGE_PRINT_QUOTE_PACKAGES  | Print quote packages when connected, `true` or `false` (Default: `true`)        |
| LONGBRIDGE_LOG_PATH              | Set the path of the log files (Default: `no logs`)                              |
| LONGBRIDGE_PAPERTRADING          | Enable paper trading mode, `true` or `false` (Default: `false`). See [Paper Trading](#paper-trading). |

Then create a config from the environment:

```javascript
const { Config } = require('longbridge');
const config = Config.fromApikeyEnv();
```

## Quote API _(Get basic information of securities)_

```javascript
const { OAuth, Config, QuoteContext } = require('longbridge');

async function main() {
  const oauth = await OAuth.build(
    "your-client-id",
    (_, url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
  const ctx = QuoteContext.new(config);
  const resp = await ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"]);
  for (let obj of resp) {
    console.log(obj.toString());
  }
}

main();
```

## Quote API _(Subscribe quotes)_

```javascript
const { OAuth, Config, QuoteContext, SubType } = require('longbridge');

async function main() {
  const oauth = await OAuth.build(
    "your-client-id",
    (_, url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
  const ctx = QuoteContext.new(config);
  ctx.setOnQuote((_, event) => console.log(event.toString()));
  await ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
    [SubType.Quote],
  );
}

main();
```

## Trade API _(Submit order)_

```javascript
const {
  OAuth,
  Config,
  TradeContext,
  Decimal,
  OrderSide,
  TimeInForceType,
  OrderType,
} = require('longbridge');

async function main() {
  const oauth = await OAuth.build(
    "your-client-id",
    (_, url) => console.log("Open this URL to authorize: " + url)
  );
  const config = Config.fromOAuth(oauth);
  const ctx = TradeContext.new(config);
  const resp = await ctx.submitOrder({
    symbol: "700.HK",
    orderType: OrderType.LO,
    side: OrderSide.Buy,
    timeInForce: TimeInForceType.Day,
    submittedPrice: new Decimal("50"),
    submittedQuantity: new Decimal(200),
  });
  console.log(resp.toString());
}

main();
```

## Paper Trading

If your account is a **paper trading** (simulation) account and you want to
place orders, you must explicitly enable paper trading mode.  Every request
then carries `x-papertrading: true`, which asks the server to serve the paper
trading account and to reject a token that belongs to a real-money account.

By default this option is **off**: the server imposes no restrictions and
accepts requests from both paper trading and real-money accounts.

> **This is a server-side check, not a client-side guarantee.** Two limits
> apply today, and neither is visible in the response:
>
> - **The US data center (LBUS) has no paper trading environment.** There is no
>   paper trading account to serve, and the server answers with the
>   **real-money** account instead of rejecting the request. The SDK therefore
>   refuses order submission and replacement locally; reads are not refused and
>   return real-money data.
> - **Under OAuth authentication the server does not apply the check at all**,
>   in any region: a real-money token is accepted and reads return real-money
>   data without an error.
>
> Do not rely on this option as the only thing standing between your strategy
> and your real-money account.

**Via environment variable** (applies automatically to all constructor methods):

```bash
export LONGBRIDGE_PAPERTRADING=true   # macOS / Linux
```

```powershell
$env:LONGBRIDGE_PAPERTRADING = "true"   # Windows PowerShell
```

**Programmatically:**

```javascript
const { Config } = require('longbridge');

const config = Config.fromApikey(
  process.env.LONGBRIDGE_APP_KEY,
  process.env.LONGBRIDGE_APP_SECRET,
  process.env.LONGBRIDGE_ACCESS_TOKEN,
  { enablePapertrading: true },
);
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If the script exits, you won't receive push events; keep Node running.
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
