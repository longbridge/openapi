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

  HttpClient http_cli;
  Status status = http_cli.from_env();
  if (!status) {
    std::cout << "failed to load configuration from environment: "
              << *status.message() << std::endl;
    return -1;
  }

  http_cli.request("get",
                   "/v1/trade/execution/today",
                   std::nullopt,
                   std::nullopt,
                   [](auto res) {
                     if (!res) {
                       std::cout << "failed: " << *res.status().message()
                                 << std::endl;
                       return;
                     }
                     std::cout << res->response_body << std::endl;
                   });

  std::cin.get();
  return 0;
}
