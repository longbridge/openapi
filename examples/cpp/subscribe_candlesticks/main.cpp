#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::quote;

static QuoteContext g_ctx;

static void
run(OAuthToken token)
{
  Config config = Config::from_oauth(token);

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

  std::string err;
  OAuthToken token = OAuthToken::load(err);
  if (err.empty()) {
    run(std::move(token));
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
        std::string save_err;
        res->save(save_err);
        if (!save_err.empty()) {
          std::cout << "failed to save token: " << save_err << std::endl;
        }
        run(std::move(*res));
      });
  }

  std::cin.get();
  return 0;
}
