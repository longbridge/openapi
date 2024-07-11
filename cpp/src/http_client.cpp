#include <vector>

#include "callback.hpp"
#include "http_client.hpp"
#include "longport.h"

namespace longport {

HttpClient::HttpClient()
  : http_client_(nullptr)
{
}

HttpClient::HttpClient(const std::string& http_url,
                       const std::string& app_key,
                       const std::string& app_secret,
                       const std::string& access_token)
{
  http_client_ = lb_http_client_new(http_url.c_str(),
                                    app_key.c_str(),
                                    app_secret.c_str(),
                                    access_token.c_str());
}

HttpClient::~HttpClient()
{
  if (http_client_) {
    lb_http_client_free(http_client_);
  }
}

Status
HttpClient::from_env()
{
  lb_error_t* err = nullptr;
  lb_http_client_t* http_client_ptr = lb_http_client_from_env(&err);
  Status status(err);
  if (status.is_ok()) {
    http_client_ = http_client_ptr;
  }
  return status;
}

void
HttpClient::request(
  const std::string& method,
  const std::string& path,
  const std::optional<std::map<std::string, std::string>>& headers,
  const std::optional<std::string>& body,
  AsyncCallback<void*, HttpResult> callback)
{
  std::vector<lb_http_header_t> c_headers;
  if (headers) {
    for (auto it = headers->begin(); it != headers->end(); it++) {
      c_headers.push_back(
        lb_http_header_t{ it->first.c_str(), it->second.c_str() });
    }
  }
  c_headers.push_back(lb_http_header_t{ nullptr, nullptr });

  lb_http_client_request(
    http_client_,
    method.c_str(),
    path.c_str(),
    c_headers.data(),
    body ? body->c_str() : nullptr,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<void*, HttpResult>(res->userdata);
      Status status(res->error);

      if (status) {
        const lb_http_result_t* result = (const lb_http_result_t*)res->data;
        HttpResult http_res(lb_http_result_response_body(result));
        (*callback_ptr)(AsyncResult<void*, HttpResult>(
          nullptr, std::move(status), &http_res));
      } else {
        (*callback_ptr)(
          AsyncResult<void*, HttpResult>(nullptr, std::move(status), nullptr));
      }
    },
    new AsyncCallback<void*, HttpResult>(callback));
}

} // namespace longport
