#include <iostream>

#include "longbridge.hpp"

using namespace longbridge;
using namespace longbridge::quote;

static void
run(const OAuth& oauth)
{
  longbridge::Config config = longbridge::Config::from_oauth(oauth);

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: "
                << *res.status().message() << std::endl;
      return;
    }

    DateTime datetime = {
      { 2025, 8, 1 },
      { 0, 0, 0 },
    };

    res.context().history_candlesticks_by_offset(
      "700.HK",
      Period::Day,
      AdjustType::NoAdjust,
      false,
      datetime,
      10,
      TradeSessions::All,
      [](auto res) {
        if (!res) {
          std::cout << "failed to request history candlesticks: "
                    << *res.status().message() << std::endl;
          return;
        }

        for (auto it = res->cbegin(); it != res->cend(); ++it) {
          std::cout << " close=" << (double)it->close
                    << " open=" << (double)it->open
                    << " low=" << (double)it->low
                    << " high=" << (double)it->high
                    << " volume=" << (int64_t)it->volume
                    << " turnover=" << (double)it->turnover
                    << " timestamp=" << (int64_t)it->timestamp << std::endl;
        }
      });
  });
}

int
main()
{
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
