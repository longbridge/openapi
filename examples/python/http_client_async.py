"""HTTP client (async). Use asyncio with HttpClient.request_async."""
import asyncio

from longport.openapi import HttpClient, OAuth


async def main() -> None:
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    http_cli = HttpClient.from_oauth(oauth.client_id, token.access_token)
    resp = await http_cli.request_async(
        "get",
        "/v1/trade/execution/today",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
