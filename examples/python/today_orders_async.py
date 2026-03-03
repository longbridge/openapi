"""Get today orders (async). Use asyncio with AsyncTradeContext."""
import asyncio

from longport.openapi import AsyncTradeContext, Config, OAuth, OAuthToken


async def main() -> None:
    try:
        token = OAuthToken.load()
    except Exception:
        oauth = OAuth("your-client-id")
        token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
        token.save()
    config = Config.from_oauth(token)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.today_orders()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
