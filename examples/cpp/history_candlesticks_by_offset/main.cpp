#include <iostream>

#include "longport.hpp"

using namespace longport;
using namespace longport::quote;

int
main()
{
  longport::Config config;
  longport::Status status = longport::Config::from_env(config);

  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << *status.message() << std::endl;
    return -1;
  }
  QuoteContext::create(config, [&](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << *res.status().message()
                << std::endl;
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
      [&](auto res) {
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

  std::cin.get();
  return 0;
}
