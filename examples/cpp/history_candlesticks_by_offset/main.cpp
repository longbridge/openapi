#include <iostream>

#include "longport.hpp"

using namespace longport;
using namespace longport::quote;

static void
run(const OAuthToken& token)
{
  longport::Config config = longport::Config::from_oauth(token);

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
  OAuthToken token;
  Status load_status = OAuthToken::load(token);
  if (load_status) {
    run(token);
  } else {
    const std::string client_id = "your-client-id";
    OAuth oauth(client_id);
    oauth.authorize(
      [](const std::string& url) {
        std::cout << "Open this URL to authorize: " << url << std::endl;
      },
      [](auto res) {
        if (!res) {
          std::cout << "authorization failed: " << *res.status().message()
                    << std::endl;
          return;
        }
        Status save_status = res->save();
        if (!save_status) {
          std::cout << "failed to save token: "
                    << *save_status.message() << std::endl;
        }
        run(*res);
      });
  }

  std::cin.get();
  return 0;
}
