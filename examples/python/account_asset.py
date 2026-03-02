import asyncio

from longport.openapi import TradeContext, Config, OAuth


async def get_config() -> Config:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return Config.from_oauth("your-client-id", token.access_token)


config = asyncio.run(get_config())
ctx = TradeContext(config)
resp = ctx.account_balance()
print(resp)
