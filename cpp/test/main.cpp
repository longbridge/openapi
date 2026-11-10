#include <iostream>
#include <longbridge.hpp>
#include <stdexcept>
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
check_trade_status(bool condition)
{
  if (!condition) {
    throw std::runtime_error("market trade-status check failed");
  }
}

static void
test_market_trade_status()
{
  using market::TradeStatus;

  check_trade_status(market::code(TradeStatus::US_TRADING) == 202);
  check_trade_status(market::code(market::normalize(TradeStatus::US_CLEAN)) ==
                     201);
  check_trade_status(market::normalize(TradeStatus::CLEAN) ==
                     TradeStatus::CLOSING);
  check_trade_status(market::label(TradeStatus::US_CLEAN) ==
                     std::string("Pre-Market"));
  check_trade_status(std::string(market::label(TradeStatus::OPEN_BID)).empty());
  check_trade_status(market::name(TradeStatus::REALTIME_QUOTE) ==
                     std::string("Temporary Break"));
  check_trade_status(market::name(TradeStatus::UNITED) ==
                     std::string("Not Listed"));
  check_trade_status(market::name(TradeStatus::TRADING_HALT) ==
                     std::string("Terminated"));
  check_trade_status(market::name(TradeStatus::FUSE) == std::string("Fuse"));
  check_trade_status(market::is_us_market(TradeStatus::US_TRADING));
  check_trade_status(market::is_us_pre_post(TradeStatus::US_AFTER));
  check_trade_status(market::is_us_night(TradeStatus::US_NIGHT));
  check_trade_status(market::is_us_closing(TradeStatus::US_CLOSING));
  check_trade_status(market::is_closing(TradeStatus::HALF_CLOSING));
  check_trade_status(market::is_us_prev(TradeStatus::US_CLEAN));
  check_trade_status(market::is_us_after(TradeStatus::US_AFTER));
  check_trade_status(market::is_trading(TradeStatus::US_AFTER_MARKET_CLEAN));
  check_trade_status(market::is_dark(TradeStatus::DARK_TRADING));
  check_trade_status(market::allow_trading(TradeStatus::NOON_CLOSING));
  check_trade_status(market::is_special(TradeStatus::TRADING_HALT));
  check_trade_status(market::from_trade_status_code(456) ==
                     TradeStatus::UNKNOWN);
  check_trade_status(market::from_trade_status_code(2001) == TradeStatus::FUSE);
  check_trade_status(market::trade_status_as_string(TradeStatus::US_CLEAN) ==
                     std::string("US_CLEAN"));
  check_trade_status(market::trade_status_as_string(TradeStatus::FUSE) ==
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

  agent::AgentContext agent_ctx = agent::AgentContext::create(config);

  agent_ctx.workspaces([](auto res) {
    if (!res) {
      std::cout << "failed to list workspaces: " << *res.status().message()
                << std::endl;
      return;
    }
    std::cout << "workspaces: " << res->workspaces.size() << std::endl;
  });

  std::cin.get();
  return 0;
}
