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

  const std::string client_id = "your-client-id";
  OAuth oauth(client_id);

  oauth.authorize(
    [](const std::string& url) { std::cout << url << std::endl; },
    [client_id](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }

      HttpClient http_cli = HttpClient::from_oauth(client_id, res->access_token());

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
    });

  std::cin.get();
  return 0;
}
