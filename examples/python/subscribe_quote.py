import asyncio
from time import sleep

from longport.openapi import QuoteContext, Config, OAuth, OAuthToken, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


async def get_token() -> OAuthToken:
    client_id = "your-client-id"
    try:
        token = OAuthToken.load()
        if token.is_expired():
            raise Exception("token expired")
        if token.expires_soon():
            oauth = OAuth(client_id)
            try:
                token = await oauth.refresh(token)
                token.save()
                return token
            except Exception:
                pass  # fall through to re-authorize
        else:
            return token
    except Exception:
        pass
    oauth = OAuth(client_id)
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    token.save()
    return token


async def get_config() -> Config:
    return Config.from_oauth(await get_token())


config = asyncio.run(get_config())
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)
ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
    [SubType.Quote],
    is_first_push=True,
)
sleep(10)
