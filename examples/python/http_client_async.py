"""HTTP client (async). Use asyncio with HttpClient.request_async."""
import asyncio

from longbridge.openapi import HttpClient, OAuthBuilder


async def main() -> None:
    oauth = await OAuthBuilder("your-client-id").build_async(
        lambda url: print(f"Open this URL to authorize: {url}")
    )
    http_cli = HttpClient.from_oauth(oauth)
    resp = await http_cli.request_async(
        "get",
        "/v1/trade/execution/today",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
