import asyncio

from longport.openapi import HttpClient, OAuth


async def get_http_client() -> HttpClient:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    return HttpClient.from_oauth(oauth.client_id, token.access_token)


http_cli = asyncio.run(get_http_client())
resp = http_cli.request(
    "get",
    "/v1/trade/execution/today",
)
print(resp)
