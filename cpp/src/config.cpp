#include "config.hpp"
#include "callback.hpp"
#include "convert.hpp"
#include "longbridge.h"
#include "oauth.hpp"

namespace longbridge {

Config::Config()
{
  config_ = nullptr;
}

Config::Config(lb_config_t* config)
{
  config_ = config;
}

Config::Config(Config&& other)
{
  config_ = other.config_;
  other.config_ = nullptr;
}

Config::~Config()
{
  if (config_) {
    lb_config_free(config_);
  }
}

Config::operator const lb_config_t*() const
{
  return config_;
}

Config
Config::from_apikey(const std::string& app_key,
                    const std::string& app_secret,
                    const std::string& access_token)
{
  return Config(lb_config_from_apikey(
    app_key.c_str(), app_secret.c_str(), access_token.c_str()));
}

Config
Config::from_apikey_env(Status& status)
{
  lb_error_t* err = nullptr;
  lb_config_t* config_ptr = lb_config_from_apikey_env(&err);
  status = std::move(Status(err));
  if (status.is_ok()) {
    return Config(config_ptr);
  }
  return Config();
}

Config
Config::from_oauth(const OAuth& oauth)
{
  return Config(lb_config_from_oauth(oauth));
}

// ── Chainable setters ─────────────────────────────────────────────────────────

Config&
Config::set_http_url(const std::string& url)
{
  lb_config_set_http_url(config_, url.c_str());
  return *this;
}

Config&
Config::set_quote_ws_url(const std::string& url)
{
  lb_config_set_quote_ws_url(config_, url.c_str());
  return *this;
}

Config&
Config::set_trade_ws_url(const std::string& url)
{
  lb_config_set_trade_ws_url(config_, url.c_str());
  return *this;
}

Config&
Config::set_language(Language language)
{
  lb_config_set_language(config_, convert::convert(language));
  return *this;
}

Config&
Config::enable_overnight()
{
  lb_config_enable_overnight(config_);
  return *this;
}

Config&
Config::enable_papertrading()
{
  lb_config_enable_papertrading(config_);
  return *this;
}

Config&
Config::set_push_candlestick_mode(PushCandlestickMode mode)
{
  lb_config_set_push_candlestick_mode(config_, convert::convert(mode));
  return *this;
}

Config&
Config::disable_print_quote_packages()
{
  lb_config_disable_print_quote_packages(config_);
  return *this;
}

Config&
Config::set_log_path(const std::string& path)
{
  lb_config_set_log_path(config_, path.c_str());
  return *this;
}

void
Config::refresh_access_token(int64_t expired_at,
                             AsyncCallback<NoContext, std::string> callback)
{
  lb_config_refresh_access_token(
    config_,
    expired_at,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<NoContext, std::string>(res->userdata);
      Status status(res->error);

      if (status) {
        std::string access_token = (const char*)res->data;

        (*callback_ptr)(AsyncResult<NoContext, std::string>(
          NoContext{}, std::move(status), &access_token));
      } else {
        (*callback_ptr)(
          AsyncResult<NoContext, std::string>(NoContext{}, std::move(status), nullptr));
      }
    },
    new AsyncCallback<NoContext, std::string>(callback));
}

} // namespace longbridge
