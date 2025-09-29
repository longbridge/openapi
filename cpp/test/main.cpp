#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  Config config;
  Status status = Config::from_env(config);
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << *status.message() << std::endl;
    return -1;
  }
  quote::QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: " << *res.status().message()
                << std::endl;
      return;
    }

    res.context().set_on_quote([](auto event) {
      std::cout << event->symbol << ": " << event->last_done.to_double()
                << std::endl;
    });

    res.context().subscribe(
      { "700.HK" }, quote::SubFlags::QUOTE(), true, [](auto res) {
        if (!res) {
          std::cout << "failed to subscribe: " << *res.status().message()
                    << std::endl;
        }
      });
  });

  std::cin.get();
  return 0;
}
