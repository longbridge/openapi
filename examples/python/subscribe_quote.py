from time import sleep

from longbridge.openapi import QuoteContext, Config, OAuthBuilder, SubType, PushQuote


def on_quote(symbol: str, event: PushQuote):
    print(symbol, event)


oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
config = Config.from_oauth(oauth)
ctx = QuoteContext(config)
ctx.set_on_quote(on_quote)
ctx.subscribe(
    ["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"],
    [SubType.Quote],
    is_first_push=True,
)
sleep(10)
