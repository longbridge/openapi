"""HTTP client (async). Use asyncio with HttpClient.request_async."""
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


async def main() -> None:
    http_cli = HttpClient.from_oauth(await get_token())
    resp = await http_cli.request_async(
        "get",
        "/v1/trade/execution/today",
    )
    print(resp)


if __name__ == "__main__":
    asyncio.run(main())
