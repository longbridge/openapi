"""Get history candlesticks (async). Use asyncio with AsyncQuoteContext."""
import asyncio
import datetime

from longport.openapi import (
    AsyncQuoteContext,
    Config,
    OAuth,
    OAuthToken,
    Period,
    AdjustType,
    TradeSessions,
)


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
    ctx = await AsyncQuoteContext.create(config)

    # get candlesticks by offset
    print("get candlesticks by offset")
    print("====================")
    candlesticks = await ctx.history_candlesticks_by_offset(
        "700.HK",
        Period.Day,
        AdjustType.NoAdjust,
        False,
        10,
        datetime.datetime(2023, 8, 18),
        TradeSessions.Intraday,
    )
    for candlestick in candlesticks:
        print(candlestick)

    # get candlesticks by date
    print("get candlesticks by date")
    print("====================")
    candlesticks = await ctx.history_candlesticks_by_date(
        "700.HK",
        Period.Day,
        AdjustType.NoAdjust,
        datetime.date(2022, 5, 5),
        datetime.date(2022, 6, 23),
        TradeSessions.Intraday,
    )
    for candlestick in candlesticks:
        print(candlestick)


if __name__ == "__main__":
    asyncio.run(main())
