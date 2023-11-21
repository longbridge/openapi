from longport.openapi import HttpClient

http_cli = HttpClient.from_env()
resp = http_cli.request("get", "/v1/trade/execution/today")
print(resp)
