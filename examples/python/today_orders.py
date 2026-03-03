import asyncio

from longport.openapi import TradeContext, Config, OAuth, OAuthToken


async def get_config() -> Config:
    try:
        token = OAuthToken.load()
    except Exception:
        oauth = OAuth("your-client-id")
        token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
        token.save()
    return Config.from_oauth(token)


config = asyncio.run(get_config())
ctx = TradeContext(config)
resp = ctx.today_orders()
print(resp)
