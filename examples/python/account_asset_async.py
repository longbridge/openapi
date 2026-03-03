import asyncio

from longport.openapi import AsyncTradeContext, Config, OAuth


async def main() -> None:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    config = Config.from_oauth(token)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.account_balance()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
