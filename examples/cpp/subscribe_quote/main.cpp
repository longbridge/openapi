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

  QuoteContext ctx;

  QuoteContext::create(config, [&](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << *res.status().message()
                << std::endl;
      return;
    }

    ctx = res.context();

    res.context().set_on_quote([](auto event) {
      std::cout << event->symbol << " timestamp=" << event->timestamp
                << " last_done=" << (double)event->last_done
                << " open=" << (double)event->open
                << " high=" << (double)event->high
                << " low=" << (double)event->low << " volume=" << event->volume
                << " turnover=" << (double)event->turnover << std::endl;
    });

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };

    res.context().subscribe(symbols, SubFlags::QUOTE(), true, [](auto res) {
      if (!res) {
        std::cout << "failed to subscribe quote: " << *res.status().message()
                  << std::endl;
        return;
      }
    });
  });

  std::cin.get();
  return 0;
}
