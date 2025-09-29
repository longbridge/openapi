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

    res.context().today_orders(std::nullopt, [](auto res) {
      if (!res) {
        std::cout << "failed to get today orders: " << *res.status().message()
                  << std::endl;
        return;
      }

      for (auto it = res->cbegin(); it != res->cend(); ++it) {
        std::cout << "order_id=" << it->order_id << " quantity=" << it->quantity
                  << " submitted_at=" << it->submitted_at << std::endl;
      }
    });
  });

  std::cin.get();
  return 0;
}
