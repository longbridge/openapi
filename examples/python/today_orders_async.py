"""Get today orders (async). Use asyncio with AsyncTradeContext."""
import asyncio

from longbridge.openapi import AsyncTradeContext, Config, OAuthBuilder


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.today_orders()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
