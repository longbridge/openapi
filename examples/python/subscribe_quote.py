import asyncio
from time import sleep

from longport.openapi import QuoteContext, Config, OAuth, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


async def get_config() -> Config:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return Config.from_oauth("your-client-id", token.access_token)


config = asyncio.run(get_config())
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)
ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
    [SubType.Quote],
    is_first_push=True,
)
sleep(10)
