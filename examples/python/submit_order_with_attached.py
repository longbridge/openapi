from decimal import Decimal

from longbridge.openapi import (
    AttachedOrderType,
    Config,
    OAuthBuilder,
    OrderSide,
    OrderType,
    SubmitAttachedParams,
    TimeInForceType,
    TradeContext,
)

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
ctx = TradeContext(config)

resp = ctx.submit_order(
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
