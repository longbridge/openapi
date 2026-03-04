# Longbridge OpenAPI SDK for Python

`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).

## Documentation

- SDK docs: https://longbridge.github.io/openapi/python/index.html
- Longbridge OpenAPI: https://open.longbridge.com/en/

## Examples

Runnable examples live in `examples/python/`, grouped as follows.

**Synchronous API** (same as the snippets in this README):

- `examples/python/account_asset.py`
- `examples/python/history_candlesticks.py`
- `examples/python/http_client.py`
- `examples/python/subscribe_candlesticks.py`
- `examples/python/subscribe_quote.py`
- `examples/python/submit_order.py`
- `examples/python/today_orders.py`

**Asynchronous API** (`AsyncQuoteContext`, `AsyncTradeContext`, `HttpClient.request_async`):

- `examples/python/account_asset_async.py`
- `examples/python/history_candlesticks_async.py`
- `examples/python/http_client_async.py`
- `examples/python/subscribe_candlesticks_async.py`
- `examples/python/subscribe_quote_async.py`
- `examples/python/submit_order_async.py`
- `examples/python/today_orders_async.py`

## References

- [Config](https://longbridge.github.io/openapi/python/config/)

  The configuration of the SDK.
   
- [QuoteContext](https://longbridge.github.io/openapi/python/quote_context/)

  The Quote API part of the SDK, e.g.: get basic information of securities, subscribe quotes...

- [TradeContext](https://longbridge.github.io/openapi/python/trade_context/)

  The Trade API part of the SDK, e.g.: submit order, get order status...

## Quickstart

_Install Longbridge OpenAPI SDK_

```bash
pip install longbridge
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

`OAuthBuilder` loads a cached token from
`~/.longbridge-openapi/tokens/<client_id>`
(`%USERPROFILE%\.longbridge-openapi\tokens\<client_id>` on Windows) if one
exists and is still valid, or starts the browser authorization flow
automatically.  The token is persisted to the same path after a successful
authorization or refresh.

```python
from longbridge.openapi import OAuthBuilder, Config

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
```

For async code use `build_async`:

```python
import asyncio
from longbridge.openapi import OAuthBuilder, Config

async def main():
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)

asyncio.run(main())
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables (macOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables (Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

Then create a config from the environment:

```python
from longbridge.openapi import Config

config = Config.from_apikey_env()
```

## Quote API _(Get basic information of securities)_

```python
from longbridge.openapi import Config, QuoteContext, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# Create a context for quote APIs
ctx = QuoteContext(config)

# Get basic information of securities
resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
print(resp)
```

## Quote API _(Subscribe quotes)_

```python
from time import sleep
from longbridge.openapi import Config, QuoteContext, SubType, PushQuote, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# A callback to receive quote data
def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

# Create a context for quote APIs
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)

# Subscribe
ctx.subscribe(["700.HK"], [SubType.Quote], is_first_push=True)

# Receive push for 30 seconds
sleep(30)
```

## Trade API _(Submit order)_

```python
from decimal import Decimal
from longbridge.openapi import TradeContext, Config, OrderType, OrderSide, TimeInForceType, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)

# Create a context for trade APIs
ctx = TradeContext(config)

# Submit order
resp = ctx.submit_order(
    "700.HK", OrderType.LO, OrderSide.Buy,
    Decimal("500"), TimeInForceType.Day,
    submitted_price=Decimal("50"),
    remark="Hello from Python SDK",
)
print(resp)
```

## Asynchronous API

The SDK provides async contexts and an async HTTP client for use with Python's `asyncio`. All I/O methods return awaitables; callbacks (e.g. for push events) are set the same way as in the sync API.

- **Async quote**: create with `ctx = await AsyncQuoteContext.create(config)`, then e.g. `await ctx.quote(["700.HK"])`, `await ctx.subscribe(...)`.
- **Async trade**: create with `ctx = await AsyncTradeContext.create(config)`, then e.g. `await ctx.today_orders()`, `await ctx.submit_order(...)`.
- **Async HTTP**: `resp = await http_cli.request_async("get", "/v1/trade/execution/today")`.

Example (async quote):

```python
import asyncio
from longbridge.openapi import Config, AsyncQuoteContext, SubType, PushQuote, OAuthBuilder

def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

async def main():
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncQuoteContext.create(config)
    ctx.set_on_quote(on_quote)
    await ctx.subscribe(["700.HK", "AAPL.US"], [SubType.Quote])
    quotes = await ctx.quote(["700.HK"])
    print(quotes)
    await asyncio.sleep(10)

asyncio.run(main())
```

See the `*_async.py` examples in `examples/python/` for full async flows.

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If the program exits, you won't receive push events; keep the process alive (e.g. `sleep(...)`).
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
