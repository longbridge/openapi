"""Submit order with attached take-profit/stop-loss (async)."""
import asyncio
from decimal import Decimal

from longbridge.openapi import (
    AsyncTradeContext,
    AttachedOrderType,
    Config,
    OAuthBuilder,
    OrderSide,
    OrderType,
    SubmitAttachedParams,
    TimeInForceType,
)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = AsyncTradeContext.create(config)

    resp = await ctx.submit_order(
        symbol="AAPL.US",
        order_type=OrderType.LO,
        side=OrderSide.Buy,
        submitted_quantity=Decimal(1),
        time_in_force=TimeInForceType.Day,
        submitted_price=Decimal(200),
        attached_params=SubmitAttachedParams(
            AttachedOrderType.Bracket,
            profit_taker_price=Decimal(220),
            stop_loss_price=Decimal(180),
        ),
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
