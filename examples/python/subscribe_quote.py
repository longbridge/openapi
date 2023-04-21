from time import sleep
from longbridge.openapi import QuoteContext, Config, SubType, PushQuote, Period, AdjustType


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


config = Config.from_env()
ctx = QuoteContext(config)
candlesticks = ctx.candlesticks(
    "SPY.US", Period.Min_5, 1000, adjust_type=AdjustType.NoAdjust)
for candlestick in candlesticks:
    print(candlestick)
