#include <iostream>
#include <longbridge.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;
using namespace longbridge::quote;

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
                  << " high=" << (double)it->high
                  << " low=" << (double)it->low
                  << " volume=" << it->volume
                  << " turnover=" << (double)it->turnover << std::endl;
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
