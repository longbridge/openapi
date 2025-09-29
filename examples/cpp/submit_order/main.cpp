#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::trade;

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << *status.message() << std::endl;
    return -1;
  }

  TradeContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create trade context: " << *res.status().message()
                << std::endl;
      return;
    }

    SubmitOrderOptions opts{
      "700.HK",     OrderType::LO,        OrderSide::Buy,
      Decimal(200), TimeInForceType::Day, Decimal(50.0),
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt,
    };
    res.context().submit_order(opts, [](auto res) {
      if (!res) {
        std::cout << "failed to submit order: " << *res.status().message()
                  << std::endl;
        return;
      }
      std::cout << "order id: " << res->order_id << std::endl;
    });
  });

  std::cin.get();
  return 0;
}
