from decimal import Decimal
from longport.openapi import TradeContext, Config, OrderSide, OrderType, TimeInForceType

config = Config.from_env()
ctx = TradeContext(config)

resp = ctx.submit_order(
    side=OrderSide.Buy,
    symbol="700.HK",
    order_type=OrderType.LO,
    submitted_price=Decimal("50"),
    submitted_quantity=200,
    time_in_force=TimeInForceType.Day,
    remark="Hello from Python SDK",
)
print(resp)
