#include "config.hpp"
#include "callback.hpp"
#include "convert.hpp"
#include "longport.h"
#include "oauth.hpp"

namespace longport {

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
                    const std::string& access_token,
                    const std::optional<std::string>& http_url,
                    const std::optional<std::string>& quote_ws_url,
                    const std::optional<std::string>& trade_ws_url,
                    const std::optional<Language>& language,
                    bool enable_overnight,
                    const std::optional<PushCandlestickMode>& push_candlestick_mode,
                    bool enable_print_quote_packages,
                    const std::optional<std::string>& log_path)
{
  lb_language_t c_language;
  if (language) {
    c_language = convert::convert(*language);
  }

  lb_push_candlestick_mode_t c_push_candlestick_mode;
  if (push_candlestick_mode) {
    c_push_candlestick_mode = convert::convert(*push_candlestick_mode);
  }

  return Config(
    lb_config_from_apikey(app_key.c_str(),
                          app_secret.c_str(),
                          access_token.c_str(),
                          http_url ? http_url->c_str() : nullptr,
                          quote_ws_url ? quote_ws_url->c_str() : nullptr,
                          trade_ws_url ? trade_ws_url->c_str() : nullptr,
                          language ? &c_language : nullptr,
                          enable_overnight,
                          push_candlestick_mode ? &c_push_candlestick_mode
                                                : nullptr,
                          enable_print_quote_packages,
                          log_path ? log_path->c_str() : nullptr));
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

} // namespace longport
