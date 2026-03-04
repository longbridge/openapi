#include <iostream>
#include <longbridge.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;
using namespace longbridge::quote;

static QuoteContext g_ctx;

static void
run(const OAuth& oauth)
{
  Config config = Config::from_oauth(oauth);

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: "
                << *res.status().message() << std::endl;
      return;
    }

    g_ctx = res.context();

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
          std::cout << "failed to subscribe quote: "
                    << *res.status().message() << std::endl;
          return;
        }
      });
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
