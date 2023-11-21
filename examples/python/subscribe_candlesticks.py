from time import sleep
from longport.openapi import QuoteContext, Config, Period, PushCandlestick


def on_candlestick(symbol: str, event: PushCandlestick):
    print(symbol, event)


config = Config.from_env()
ctx = QuoteContext(config)
ctx.set_on_candlestick(on_candlestick)

ctx.subscribe_candlesticks("AAPL.US", Period.Min_1)
sleep(30)
