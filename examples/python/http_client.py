import asyncio

from longport.openapi import HttpClient, OAuth, OAuthToken


async def get_http_client() -> HttpClient:
    try:
        token = OAuthToken.load()
    except Exception:
        oauth = OAuth("your-client-id")
        token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
        token.save()
    return HttpClient.from_oauth(token)


http_cli = asyncio.run(get_http_client())
resp = http_cli.request(
    "get",
    "/v1/trade/execution/today",
)
print(resp)
