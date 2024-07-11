#pragma once

#include <map>
#include <optional>
#include <string>

#include "async_result.hpp"
#include "status.hpp"

typedef struct lb_http_client_t lb_http_client_t;

namespace longport {

struct HttpResult
{
  const char* response_body;

  HttpResult(const char* response_body) {}
};

class HttpClient
{
private:
  lb_http_client_t* http_client_;

public:
  HttpClient();
  HttpClient(HttpClient&) = delete;
  ~HttpClient();

  /**
   * HttpClient
   *
   * @param http_url HTTP endpoint url
   * @param app_key App key
   * @param app_secret App secret
   * @param access_token Access token
   */
  HttpClient(const std::string& http_url,
             const std::string& app_key,
             const std::string& app_secret,
             const std::string& access_token);

  Status from_env();

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