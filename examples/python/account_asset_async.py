import asyncio

from longbridge.openapi import AsyncTradeContext, Config, OAuthBuilder


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    config = Config.from_oauth(oauth)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.account_balance()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
