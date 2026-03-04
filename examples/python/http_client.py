from longbridge.openapi import HttpClient, OAuthBuilder

oauth = OAuthBuilder("your-client-id").build(
    lambda url: print(f"Open this URL to authorize: {url}")
)
http_cli = HttpClient.from_oauth(oauth)
resp = http_cli.request(
    "get",
    "/v1/trade/execution/today",
)
print(resp)
