#include <iostream>
#include <longbridge.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;
using namespace longbridge::trade;

static void
run(const OAuth& oauth)
{
  Config config = Config::from_oauth(oauth);
  TradeContext ctx = TradeContext::create(config);

  SubmitAttachedParams attached{
    AttachedOrderType::Bracket,
    Decimal("220"),  // profit_taker_price
    Decimal("180"),  // stop_loss_price
  };

  SubmitOrderOptions opts{
    "AAPL.US",    OrderType::LO,        OrderSide::Buy,
    Decimal("1"), TimeInForceType::Day, Decimal("200"),
  };
  opts.attached_params = attached;

  ctx.submit_order(opts, [](auto res) {
    if (!res) {
      std::cout << "failed to submit order: " << *res.status().message()
                << std::endl;
      return;
    }
    std::cout << "order id: " << res->order_id << std::endl;
  });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const std::string client_id = "your-client-id";

  OAuthBuilder(client_id).build(
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }
      run(*res);
    });

  std::cin.get();
  return 0;
}
