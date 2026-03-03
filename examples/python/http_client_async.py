"""HTTP client (async). Use asyncio with HttpClient.request_async."""
import asyncio

from longport.openapi import HttpClient, OAuth, OAuthToken


async def main() -> None:
    try:
        token = OAuthToken.load()
    except Exception:
        oauth = OAuth("your-client-id")
        token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
        token.save()
    http_cli = HttpClient.from_oauth(token)
    resp = await http_cli.request_async(
        "get",
        "/v1/trade/execution/today",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
