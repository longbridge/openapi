from time import sleep

from longbridge.openapi import (
    QuoteContext,
    Config,
    OAuthBuilder,
    Period,
    PushCandlestick,
    TradeSessions,
)


def on_candlestick(symbol: str, event: PushCandlestick):
    print(symbol, event)


oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
ctx = QuoteContext(config)
ctx.set_on_candlestick(on_candlestick)
ctx.subscribe_candlesticks(
    "AAPL.US",
    Period.Min_1,
    TradeSessions.Intraday,
)
sleep(30)
