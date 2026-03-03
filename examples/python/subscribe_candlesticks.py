import asyncio
from time import sleep

from longport.openapi import (
    QuoteContext,
    Config,
    OAuth,
    Period,
    PushCandlestick,
    TradeSessions,
)


def on_candlestick(symbol: str, event: PushCandlestick):
    print(symbol, event)


async def get_config() -> Config:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return Config.from_oauth(token)


config = asyncio.run(get_config())
ctx = QuoteContext(config)
ctx.set_on_candlestick(on_candlestick)
ctx.subscribe_candlesticks(
    "AAPL.US",
    Period.Min_1,
    TradeSessions.Intraday,
)
sleep(30)
