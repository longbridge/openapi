from longport.openapi import TradeContext, Config

config = Config.from_env()
ctx = TradeContext(config)

resp = ctx.account_balance()
print(resp)
