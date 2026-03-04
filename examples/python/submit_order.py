from decimal import Decimal

from longbridge.openapi import (
    TradeContext,
    Config,
    OAuthBuilder,
    OrderSide,
    OrderType,
    TimeInForceType,
    OutsideRTH,
)

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
ctx = TradeContext(config)

resp = ctx.submit_order(
    side=OrderSide.Buy,
    symbol="700.HK",
    order_type=OrderType.MO,
    submitted_quantity=Decimal(200),
    time_in_force=TimeInForceType.Day,
    outside_rth=OutsideRTH.AnyTime,
    remark="Hello from Python SDK",
)
print(resp)
