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

    res.context().set_on_candlestick([](auto event) {
      std::cout << event->symbol
                << " timestamp=" << event->candlestick.timestamp
                << " close=" << (double)event->candlestick.close
                << " open=" << (double)event->candlestick.open
                << " high=" << (double)event->candlestick.high
                << " low=" << (double)event->candlestick.low
                << " volume=" << event->candlestick.volume
                << " turnover=" << (double)event->candlestick.turnover
                << std::endl;
    });

    res.context().subscribe_candlesticks(
      "AAPL.US", Period::Min1, TradeSessions::All, [](auto res) {
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
