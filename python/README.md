# LongPort OpenAPI SDK for Python

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/python/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

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

- [Config](https://longportapp.github.io/openapi/python/config/)

  The configuration of the SDK.
   
- [QuoteContext](https://longportapp.github.io/openapi/python/quote_context/)

  The Quote API part of the SDK, e.g.: get basic information of securities, subscribe quotes...

- [TradeContext](https://longportapp.github.io/openapi/python/trade_context/)

  The Trade API part of the SDK, e.g.: submit order, get order status...

## Quickstart

_Install LongPort OpenAPI SDK_

```bash
pip install longport
```

### Authentication

LongPort OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

```python
from longport.openapi import Config, OAuth

# Start OAuth 2.0 authorization flow
oauth = OAuth("your-client-id")
token = oauth.authorize(lambda url: print(f"Please visit: {url}"))

# Create config with OAuth token
config = Config.from_oauth(token)
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```python
from longport.openapi import Config, QuoteContext, OAuth

# Authenticate with OAuth 2.0
oauth = OAuth("your-client-id")
token = oauth.authorize(lambda url: print(f"Please visit: {url}"))
config = Config.from_oauth(token)

# Create a context for quote APIs
ctx = QuoteContext(config)

# Get basic information of securities
resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
print(resp)
```

## Quote API _(Subscribe quotes)_

```python
from time import sleep
from longport.openapi import Config, QuoteContext, SubType, PushQuote, OAuth

# Authenticate with OAuth 2.0
oauth = OAuth("your-client-id")
token = oauth.authorize(lambda url: print(f"Please visit: {url}"))
config = Config.from_oauth(token)

# A callback to receive quote data
def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

# Create a context for quote APIs
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)

# Subscribe
resp = ctx.subscribe(["700.HK"], [SubType.Quote], is_first_push=True)

# Receive push duration to 30 seconds
sleep(30)
```

## Trade API _(Submit order)_

```python
from decimal import Decimal
from longport.openapi import TradeContext, Config, OrderType, OrderSide, TimeInForceType, OAuth

# Authenticate with OAuth 2.0
oauth = OAuth("your-client-id")
token = oauth.authorize(lambda url: print(f"Please visit: {url}"))
config = Config.from_oauth(token)

# Create a context for trade APIs
ctx = TradeContext(config)

# Submit order
resp = ctx.submit_order("700.HK", OrderType.LO, OrderSide.Buy, Decimal(
    "500"), TimeInForceType.Day, submitted_price=Decimal("50"), remark="Hello from Python SDK")
print(resp)
```

## Asynchronous API

**Note:** The async API is currently in an early experience stage; we welcome feedback.

The SDK provides async contexts and an async HTTP client for use with Python's `asyncio`. All I/O methods return awaitables; callbacks (e.g. for push events) are set the same way as in the sync API and may be invoked from internal threads.

- **Async quote**: create with `ctx = await AsyncQuoteContext.create(config)`, then e.g. `await ctx.quote(["700.HK"])`, `await ctx.subscribe(...)`.
- **Async trade**: create with `ctx = await AsyncTradeContext.create(config)`, then e.g. `await ctx.today_orders()`, `await ctx.submit_order(...)`.
- **Async HTTP**: `resp = await http_cli.request_async("get", "/v1/trade/execution/today")`.

Example (async quote):

```python
import asyncio
from longport.openapi import Config, AsyncQuoteContext, SubType, PushQuote, OAuth

def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)

async def main():
    oauth = OAuth("your-client-id")
    token = oauth.authorize(lambda url: print(f"Please visit: {url}"))
    config = Config.from_oauth(token)
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
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
