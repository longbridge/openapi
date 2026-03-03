import asyncio

from longport.openapi import AsyncTradeContext, Config, OAuth, OAuthToken


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


async def main() -> None:
    token = await get_token()
    config = Config.from_oauth(token)
    ctx = await AsyncTradeContext.create(config)
    resp = await ctx.account_balance()
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
