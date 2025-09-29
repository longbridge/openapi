#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::quote;

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

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << *res.status().message()
                << std::endl;
      return;
    }

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };
    res.context().quote(symbols, [](auto res) {
      if (!res) {
        std::cout << "failed to get quote: " << *res.status().message()
                  << std::endl;
        return;
      }

      for (auto it = res->cbegin(); it != res->cend(); ++it) {
        std::cout << it->symbol << " timestamp=" << it->timestamp
                  << " last_done=" << (double)it->last_done
                  << " prev_close=" << (double)it->prev_close
                  << " open=" << (double)it->open
                  << " high=" << (double)it->high << " low=" << (double)it->low
                  << " volume=" << it->volume
                  << " turnover=" << (double)it->turnover << std::endl;
      }
    });
  });

  std::cin.get();
  return 0;
}
