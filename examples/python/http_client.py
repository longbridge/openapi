import asyncio

from longport.openapi import HttpClient, OAuth, OAuthToken


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


async def get_http_client() -> HttpClient:
    return HttpClient.from_oauth(await get_token())


http_cli = asyncio.run(get_http_client())
resp = http_cli.request(
    "get",
    "/v1/trade/execution/today",
)
print(resp)
