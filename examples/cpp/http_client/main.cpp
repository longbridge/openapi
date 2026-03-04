#include <iostream>
#include <longbridge.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longbridge;

static void
run(const OAuth& oauth)
{
  HttpClient http_cli = HttpClient::from_oauth(oauth);

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
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

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
