import asyncio
import datetime

from longport.openapi import (
    QuoteContext,
    Config,
    OAuth,
    Period,
    AdjustType,
    TradeSessions,
)


async def get_config() -> Config:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return Config.from_oauth("your-client-id", token.access_token)


config = asyncio.run(get_config())
ctx = QuoteContext(config)

# get candlesticks by offset
print("get candlesticks by offset")
print("====================")
candlesticks = ctx.history_candlesticks_by_offset(
    "700.HK",
    Period.Day,
    AdjustType.NoAdjust,
    False,
    10,
    datetime.datetime(2023, 8, 18),
    TradeSessions.Intraday,
)
for candlestick in candlesticks:
    print(candlestick)

# get candlesticks by date
print("get candlesticks by date")
print("====================")
candlesticks = ctx.history_candlesticks_by_date(
    "700.HK",
    Period.Day,
    AdjustType.NoAdjust,
    datetime.date(2022, 5, 5),
    datetime.date(2022, 6, 23),
    TradeSessions.Intraday,
)
for candlestick in candlesticks:
    print(candlestick)
