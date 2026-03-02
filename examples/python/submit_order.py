import asyncio
from decimal import Decimal

from longport.openapi import (
    TradeContext,
    Config,
    OAuth,
    OrderSide,
    OrderType,
    TimeInForceType,
    OutsideRTH,
)


async def get_config() -> Config:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return Config.from_oauth("your-client-id", token.access_token)


config = asyncio.run(get_config())
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
