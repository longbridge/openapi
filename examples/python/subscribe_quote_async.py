"""Subscribe quote (async). Use asyncio with AsyncQuoteContext."""
import asyncio

from longbridge.openapi import AsyncQuoteContext, Config, OAuthBuilder, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote) -> None:
    print(symbol, event)


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncQuoteContext.create(config)
    ctx.set_on_quote(on_quote)
    await ctx.subscribe(
        ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
        [SubType.Quote],
    )
    await asyncio.sleep(10)


if __name__ == "__main__":
    asyncio.run(main())
