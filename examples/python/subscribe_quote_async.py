"""Subscribe quote (async). Use asyncio with AsyncQuoteContext."""
import asyncio

from longport.openapi import AsyncQuoteContext, Config, OAuth, OAuthToken, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote) -> None:
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
    ctx.set_on_quote(on_quote)
    await ctx.subscribe(
        ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
        [SubType.Quote],
    )
    await asyncio.sleep(10)


if __name__ == "__main__":
    asyncio.run(main())
