import asyncio

from longport.openapi import TradeContext, Config, OAuth, OAuthToken


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
ctx = TradeContext(config)
resp = ctx.account_balance()
print(resp)
