import asyncio
from longport.openapi import Config, OAuth


async def main():
    oauth = OAuth("your-client-id")
    token = await oauth.authorize(lambda url: print(f"Open this URL to authorize: {url}"))
    print(f"Access token: {token.access_token}")
    print(f"Expires at: {token.expires_at}")

    config = Config.from_oauth("your-client-id", token.access_token)
    print(f"Config created: {config}")


asyncio.run(main())
