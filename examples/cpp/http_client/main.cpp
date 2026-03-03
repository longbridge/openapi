#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;

static void
run(OAuthToken token)
{
  HttpClient http_cli = HttpClient::from_oauth(token);

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
