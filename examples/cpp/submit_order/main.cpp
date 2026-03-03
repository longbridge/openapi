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

  const std::string client_id = "your-client-id";
  OAuth oauth(client_id);

  oauth.authorize(
    [](const std::string& url) { std::cout << url << std::endl; },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }

      Config config = Config::from_oauth(*res.operator->());

      TradeContext::create(config, [](auto res) {
        if (!res) {
          std::cout << "failed to create trade context: "
                    << *res.status().message() << std::endl;
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
    });

  std::cin.get();
  return 0;
}
