#include "oauth.hpp"
#include "callback.hpp"
#include "longport.h"

namespace longport {

// ── OAuthToken ───────────────────────────────────────────────────────────────

OAuthToken&
OAuthToken::operator=(OAuthToken&& other)
{
  if (this != &other) {
    if (token_)
      lb_oauth_token_free(token_);
    token_ = other.token_;
    other.token_ = nullptr;
  }
  return *this;
}

OAuthToken::~OAuthToken()
{
  if (token_)
    lb_oauth_token_free(token_);
}

bool
OAuthToken::is_expired() const
{
  return lb_oauth_token_is_expired(token_) != 0;
}

bool
OAuthToken::expires_soon() const
{
  return lb_oauth_token_expires_soon(token_) != 0;
}

// ── OAuth ────────────────────────────────────────────────────────────────────

OAuth::OAuth(const std::string& client_id)
{
  oauth_ = lb_oauth_new(client_id.c_str());
}

OAuth::OAuth(OAuth&& other)
{
  oauth_ = other.oauth_;
  other.oauth_ = nullptr;
}

OAuth::~OAuth()
{
  if (oauth_)
    lb_oauth_free(oauth_);
}

void
OAuth::authorize(std::function<void(const std::string&)> open_url,
                 AsyncCallback<void*, OAuthToken> callback)
{
  auto* open_url_ptr = new std::function<void(const std::string&)>(open_url);

  lb_oauth_authorize(
    oauth_,
    [](const char* url, void* userdata) {
      auto* fn =
        static_cast<std::function<void(const std::string&)>*>(userdata);
      (*fn)(url);
      delete fn;
    },
    open_url_ptr,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<void*, OAuthToken>(res->userdata);
      Status status(res->error);

      if (status) {
        OAuthToken token(static_cast<lb_oauth_token_t*>(res->data));
        (*callback_ptr)(
          AsyncResult<void*, OAuthToken>(nullptr, std::move(status), &token));
      } else {
        (*callback_ptr)(
          AsyncResult<void*, OAuthToken>(nullptr, std::move(status), nullptr));
      }
    },
    new AsyncCallback<void*, OAuthToken>(callback));
}

void
OAuth::refresh(const OAuthToken& token,
               AsyncCallback<void*, OAuthToken> callback)
{
  lb_oauth_refresh(
    oauth_,
    token.get(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<void*, OAuthToken>(res->userdata);
      Status status(res->error);

      if (status) {
        OAuthToken new_token(static_cast<lb_oauth_token_t*>(res->data));
        (*callback_ptr)(AsyncResult<void*, OAuthToken>(
          nullptr, std::move(status), &new_token));
      } else {
        (*callback_ptr)(
          AsyncResult<void*, OAuthToken>(nullptr, std::move(status), nullptr));
      }
    },
    new AsyncCallback<void*, OAuthToken>(callback));
}

} // namespace longport
