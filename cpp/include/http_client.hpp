#pragma once

#include <map>
#include <optional>
#include <string>

#include "async_result.hpp"
#include "oauth.hpp"
#include "status.hpp"

typedef struct lb_http_client_t lb_http_client_t;

namespace longbridge {

struct HttpResult
{
  const char* response_body;

  HttpResult(const char* response_body)
    : response_body(response_body)
  {
  }
};

class HttpClient
{
private:
  lb_http_client_t* http_client_;

public:
  HttpClient();
  HttpClient(HttpClient&) = delete;
  HttpClient(HttpClient&&) noexcept;
  ~HttpClient();

  /**
   * Create a new `HttpClient` for API Key authentication
   *
   * @param app_key      App key
   * @param app_secret   App secret
   * @param access_token Access token
   * @param http_url     HTTP endpoint url (default: https://openapi.longbridge.com)
   */
  static HttpClient from_apikey(const std::string& app_key,
                                const std::string& app_secret,
                                const std::string& access_token,
                                const std::optional<std::string>& http_url = std::nullopt);

  /**
   * Create a new `HttpClient` from environment variables (API Key mode)
   *
   * @param status  Out-param; set to the error if creation fails
   */
  static HttpClient from_apikey_env(Status& status);

  /**
   * Create a new `HttpClient` from an OAuth 2.0 client
   *
   * @param oauth     OAuth 2.0 client obtained from `OAuthBuilder::build`
   * @param http_url  HTTP endpoint url (default: https://openapi.longbridge.com)
   */
  static HttpClient from_oauth(const OAuth& oauth,
                               const std::optional<std::string>& http_url = std::nullopt);

  /**
   * Performs a HTTP request
   */
  void request(const std::string& method,
               const std::string& path,
               const std::optional<std::map<std::string, std::string>>& headers,
               const std::optional<std::string>& body,
               AsyncCallback<void*, HttpResult> callback);
};

}
