from time import sleep
from longbridge.openapi import QuoteContext, Config, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


config = Config.from_env()
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)

symbols = ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"]
ctx.subscribe(symbols, [SubType.Quote], True)
sleep(30)
