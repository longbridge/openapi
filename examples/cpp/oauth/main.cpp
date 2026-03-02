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
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [client_id](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }

      const OAuthToken& token = *res;
      std::cout << "Access token: " << token.access_token() << std::endl;
      std::cout << "Expires at: " << token.expires_at() << std::endl;

      Config config;
      Status status = Config::from_oauth(client_id, token.access_token(), config);
      if (!status) {
        std::cout << "failed to create config: " << *status.message()
                  << std::endl;
        return;
      }

      std::cout << "Config created successfully" << std::endl;
    });

  std::cin.get();
  return 0;
}
