"""Subscribe candlesticks (async). Use asyncio with AsyncQuoteContext."""
import asyncio
from longport.openapi import (
    AsyncQuoteContext,
    Config,
    OAuth,
    OAuthToken,
    Period,
    PushCandlestick,
    TradeSessions,
)


def on_candlestick(symbol: str, event: PushCandlestick) -> None:
    print(symbol, event)


async def main() -> None:
    try:
        token = OAuthToken.load()
    except Exception:
        oauth = OAuth("your-client-id")
        token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
        token.save()
    config = Config.from_oauth(token)
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
