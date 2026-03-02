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

  const std::string client_id = "your-client-id";
  OAuth oauth(client_id);

  QuoteContext ctx;

  oauth.authorize(
    [](const std::string& url) { std::cout << url << std::endl; },
    [client_id, &ctx](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }

      Config config = Config::from_oauth(client_id, res->access_token());

      QuoteContext::create(config, [&](auto res) {
        if (!res) {
          std::cout << "failed to create quote context: "
                    << *res.status().message() << std::endl;
          return;
        }

        ctx = res.context();

        res.context().set_on_quote([](auto event) {
          std::cout << event->symbol << " timestamp=" << event->timestamp
                    << " last_done=" << (double)event->last_done
                    << " open=" << (double)event->open
                    << " high=" << (double)event->high
                    << " low=" << (double)event->low
                    << " volume=" << event->volume
                    << " turnover=" << (double)event->turnover << std::endl;
        });

        std::vector<std::string> symbols = {
          "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
        };

        res.context().subscribe(symbols, SubFlags::QUOTE(), [](auto res) {
          if (!res) {
            std::cout << "failed to subscribe quote: "
                      << *res.status().message() << std::endl;
            return;
          }
        });
      });
    });

  std::cin.get();
  return 0;
}
