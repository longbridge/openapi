"""Get history candlesticks (async). Use asyncio with AsyncQuoteContext."""
import asyncio
import datetime

from longbridge.openapi import (
    AsyncQuoteContext,
    Config,
    OAuthBuilder,
    Period,
    AdjustType,
    TradeSessions,
)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncQuoteContext.create(config)

    # get candlesticks by offset
    print("get candlesticks by offset")
    print("====================")
    candlesticks = await ctx.history_candlesticks_by_offset(
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
    candlesticks = await ctx.history_candlesticks_by_date(
        "700.HK",
        Period.Day,
        AdjustType.NoAdjust,
        datetime.date(2022, 5, 5),
        datetime.date(2022, 6, 23),
        TradeSessions.Intraday,
    )
    for candlestick in candlesticks:
        print(candlestick)


if __name__ == "__main__":
    asyncio.run(main())
