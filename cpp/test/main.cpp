#include <iostream>
#include <longbridge.hpp>
#include <cassert>
#include <type_traits>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;

static_assert(std::is_same_v<decltype(market::MarketTimeItem{}.trade_status),
                             market::TradeStatus>);
static_assert(std::is_same_v<decltype(market::MarketTimeItem{}.delay_trade_status),
                             market::TradeStatus>);

static void
test_market_trade_status()
{
  using market::TradeStatus;

  assert(market::code(TradeStatus::US_TRADING) == 202);
  assert(market::code(market::normalize(TradeStatus::US_CLEAN)) == 201);
  assert(market::normalize(TradeStatus::CLEAN) == TradeStatus::CLOSING);
  assert(market::label(TradeStatus::US_CLEAN) == std::string("Pre-Market"));
  assert(std::string(market::label(TradeStatus::OPEN_BID)).empty());
  assert(market::name(TradeStatus::REALTIME_QUOTE) ==
         std::string("Temporary Break"));
  assert(market::name(TradeStatus::UNITED) == std::string("Not Listed"));
  assert(market::name(TradeStatus::TRADING_HALT) == std::string("Terminated"));
  assert(market::name(TradeStatus::FUSE) == std::string("Fuse"));
  assert(market::is_us_market(TradeStatus::US_TRADING));
  assert(market::is_us_pre_post(TradeStatus::US_AFTER));
  assert(market::is_us_night(TradeStatus::US_NIGHT));
  assert(market::is_us_closing(TradeStatus::US_CLOSING));
  assert(market::is_closing(TradeStatus::HALF_CLOSING));
  assert(market::is_us_prev(TradeStatus::US_CLEAN));
  assert(market::is_us_after(TradeStatus::US_AFTER));
  assert(market::is_trading(TradeStatus::US_AFTER_MARKET_CLEAN));
  assert(market::is_dark(TradeStatus::DARK_TRADING));
  assert(market::allow_trading(TradeStatus::NOON_CLOSING));
  assert(market::is_special(TradeStatus::TRADING_HALT));
  assert(market::from_trade_status_code(456) == TradeStatus::UNKNOWN);
  assert(market::from_trade_status_code(2001) == TradeStatus::FUSE);
  assert(market::trade_status_as_string(TradeStatus::US_CLEAN) ==
         std::string("US_CLEAN"));
  assert(market::trade_status_as_string(TradeStatus::FUSE) ==
         std::string("FUSE"));
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  test_market_trade_status();

  Status status;
  Config config = Config::from_apikey_env(status);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << *status.message() << std::endl;
    return -1;
  }
  quote::QuoteContext ctx = quote::QuoteContext::create(config);

  ctx.set_on_quote([](auto event) {
    std::cout << event->symbol << ": " << event->last_done.to_double()
              << std::endl;
  });

  ctx.subscribe({ "700.HK" }, quote::SubFlags::QUOTE(), [](auto res) {
    if (!res) {
      std::cout << "failed to subscribe: " << *res.status().message()
                << std::endl;
    }
  });

  std::cin.get();
  return 0;
}
