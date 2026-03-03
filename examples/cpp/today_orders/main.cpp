#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::trade;

static void
run(const OAuthToken& token)
{
  Config config = Config::from_oauth(token);

  TradeContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create trade context: "
                << *res.status().message() << std::endl;
      return;
    }

    res.context().today_orders(std::nullopt, [](auto res) {
      if (!res) {
        std::cout << "failed to get today orders: "
                  << *res.status().message() << std::endl;
        return;
      }

      for (auto it = res->cbegin(); it != res->cend(); ++it) {
        std::cout << "order_id=" << it->order_id
                  << " quantity=" << it->quantity
                  << " submitted_at=" << it->submitted_at << std::endl;
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
