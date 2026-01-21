# LongPort OpenAPI SDK for Python

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/python/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

## Examples

Runnable examples live in `examples/python/`:

- `examples/python/account_asset.py`
- `examples/python/history_candlesticks.py`
- `examples/python/http_client.py`
- `examples/python/subscribe_candlesticks.py`
- `examples/python/subscribe_quote.py`
- `examples/python/submit_order.py`
- `examples/python/today_orders.py`

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
from longport.openapi import Config, QuoteContext

# Load configuration from environment variables
config = Config.from_env()

# Create a context for quote APIs
ctx = QuoteContext(config)

# Get basic information of securities
resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
print(resp)
```

## Quote API _(Subscribe quotes)_

```python
from time import sleep
from longport.openapi import Config, QuoteContext, SubType, PushQuote

# Load configuration from environment variables
config = Config.from_env()

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
from longport.openapi import TradeContext, Config, OrderType, OrderSide, TimeInForceType

# Load configuration from environment variables
config = Config.from_env()

# Create a context for trade APIs
ctx = TradeContext(config)

# Submit order
resp = ctx.submit_order("700.HK", OrderType.LO, OrderSide.Buy, Decimal(
    "500"), TimeInForceType.Day, submitted_price=Decimal("50"), remark="Hello from Python SDK")
print(resp)
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If the program exits, you won't receive push events; keep the process alive (e.g. `sleep(...)`).
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
