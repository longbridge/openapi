"""Subscribe candlesticks (async). Use asyncio with AsyncQuoteContext."""
import asyncio
from longbridge.openapi import (
    AsyncQuoteContext,
    Config,
    OAuthBuilder,
    Period,
    PushCandlestick,
    TradeSessions,
)


def on_candlestick(symbol: str, event: PushCandlestick) -> None:
    print(symbol, event)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncQuoteContext.create(config)
    ctx.set_on_candlestick(on_candlestick)
    await ctx.subscribe_candlesticks(
        "AAPL.US",
        Period.Min_1,
        TradeSessions.Intraday,
    )
    await asyncio.sleep(30)


if __name__ == "__main__":
    asyncio.run(main())
