#include "oauth.hpp"
#include "callback.hpp"
#include "longbridge.h"

namespace longbridge {

// ── OAuth ────────────────────────────────────────────────────────────────────

OAuth::OAuth(lb_oauth_t* oauth)
  : oauth_(oauth)
{
}

OAuth::OAuth(const OAuth& other)
  : oauth_(lb_oauth_clone(other.oauth_))
{
}

OAuth::OAuth(OAuth&& other)
  : oauth_(other.oauth_)
{
  other.oauth_ = nullptr;
}

OAuth&
OAuth::operator=(const OAuth& other)
{
  if (this != &other) {
    if (oauth_)
      lb_oauth_free(oauth_);
    oauth_ = lb_oauth_clone(other.oauth_);
  }
  return *this;
}

OAuth&
OAuth::operator=(OAuth&& other)
{
  if (this != &other) {
    if (oauth_)
      lb_oauth_free(oauth_);
    oauth_ = other.oauth_;
    other.oauth_ = nullptr;
  }
  return *this;
}

OAuth::~OAuth()
{
  if (oauth_)
    lb_oauth_free(oauth_);
}

// ── OAuthBuilder ─────────────────────────────────────────────────────────────

OAuthBuilder::OAuthBuilder(const std::string& client_id, uint16_t callback_port)
  : client_id_(client_id)
  , callback_port_(callback_port)
{
}

void
OAuthBuilder::build(std::function<void(const std::string&)> open_url,
                    AsyncCallback<void*, OAuth> callback)
{
  auto* open_url_ptr = new std::function<void(const std::string&)>(open_url);

  lb_oauth_new(
    client_id_.c_str(),
    callback_port_,
    [](const char* url, void* userdata) {
      auto* fn = static_cast<std::function<void(const std::string&)>*>(userdata);
      (*fn)(url);
      delete fn;
    },
    open_url_ptr,
    [](const lb_async_result_t* res) {
      auto callback_ptr = callback::get_async_callback<void*, OAuth>(res->userdata);
      Status status(res->error);

      if (status) {
        OAuth oauth(static_cast<lb_oauth_t*>(res->data));
        (*callback_ptr)(
          AsyncResult<void*, OAuth>(nullptr, std::move(status), &oauth));
      } else {
        (*callback_ptr)(
          AsyncResult<void*, OAuth>(nullptr, std::move(status), nullptr));
      }
    },
    new AsyncCallback<void*, OAuth>(callback));
}

} // namespace longbridge
