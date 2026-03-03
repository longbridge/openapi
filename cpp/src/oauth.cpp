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
  return lb_oauth_token_is_expired(token_);
}

bool
OAuthToken::expires_soon() const
{
  return lb_oauth_token_expires_soon(token_);
}

OAuthToken
OAuthToken::load(std::string& error)
{
  lb_error_t* err = nullptr;
  lb_oauth_token_t* token = lb_oauth_token_load(&err);
  if (!token) {
    error = lb_error_message(err);
    lb_error_free(err);
    return OAuthToken(nullptr);
  }
  error.clear();
  return OAuthToken(token);
}

OAuthToken
OAuthToken::load_from_path(const std::string& path, std::string& error)
{
  lb_error_t* err = nullptr;
  lb_oauth_token_t* token = lb_oauth_token_load_from_path(path.c_str(), &err);
  if (!token) {
    error = lb_error_message(err);
    lb_error_free(err);
    return OAuthToken(nullptr);
  }
  error.clear();
  return OAuthToken(token);
}

bool
OAuthToken::save(std::string& error) const
{
  lb_error_t* err = nullptr;
  bool ok = lb_oauth_token_save(token_, &err);
  if (!ok) {
    error = lb_error_message(err);
    lb_error_free(err);
  } else {
    error.clear();
  }
  return ok;
}

bool
OAuthToken::save_to_path(const std::string& path, std::string& error) const
{
  lb_error_t* err = nullptr;
  bool ok = lb_oauth_token_save_to_path(token_, path.c_str(), &err);
  if (!ok) {
    error = lb_error_message(err);
    lb_error_free(err);
  } else {
    error.clear();
  }
  return ok;
}

// ── OAuth ────────────────────────────────────────────────────────────────────

OAuth::OAuth(const std::string& client_id)
{
  oauth_ = lb_oauth_new(client_id.c_str());
}

void
OAuth::set_callback_port(uint16_t callback_port)
{
  lb_oauth_set_callback_port(oauth_, callback_port);
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
