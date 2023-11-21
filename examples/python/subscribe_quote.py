from time import sleep
from longport.openapi import QuoteContext, Config, SubType, PushQuote, Period, AdjustType


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


config = Config.from_env()
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)
ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"], [SubType.Quote], is_first_push=True)
sleep(10)
