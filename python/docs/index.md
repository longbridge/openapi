# LongPort OpenAPI SDK for Python

`longport` provides an easy-to-use interface for invokes [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## References

- [Config](reference_all.md#longport.openapi.Config)

  The configuration of the SDK.
   
- [QuoteContext](reference_all.md#longport.openapi.QuoteContext)

  The Quote API part of the SDK, e.g.: get basic information of securities, subscribe quotes...

- [TradeContext](reference_all.md#longport.openapi.TradeContext)

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

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license [LICENSE-MIT](http://opensource.org/licenses/MIT) at your option.
