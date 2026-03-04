"""Submit order (async). Use asyncio with AsyncTradeContext."""
import asyncio
from decimal import Decimal

from longbridge.openapi import (
    AsyncTradeContext,
    Config,
    OAuthBuilder,
    OrderSide,
    OrderType,
    OutsideRTH,
    TimeInForceType,
)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.submit_order(
        symbol="700.HK",
        order_type=OrderType.MO,
        side=OrderSide.Buy,
        submitted_quantity=Decimal(200),
        time_in_force=TimeInForceType.Day,
        outside_rth=OutsideRTH.AnyTime,
        remark="Hello from Python SDK",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
