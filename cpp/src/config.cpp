#include "config.hpp"
#include "longbridge.h"

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

Config::Config(const std::string& app_key,
               const std::string& app_secret,
               const std::string& access_token,
               const std::optional<std::string>& http_url,
               const std::optional<std::string>& quote_ws_url,
               const std::optional<std::string>& trade_ws_url)
{
  config_ = lb_config_new(app_key.c_str(),
                          app_secret.c_str(),
                          access_token.c_str(),
                          http_url ? http_url->c_str() : nullptr,
                          http_url ? quote_ws_url->c_str() : nullptr,
                          http_url ? trade_ws_url->c_str() : nullptr);
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

Status
Config::from_env(Config& config)
{
  lb_error_t* err = nullptr;
  lb_config_t* config_ptr = lb_config_from_env(&err);
  Status status(err);
  if (status.is_ok()) {
    config.config_ = config_ptr;
  }
  return status;
}

} // namespace longbridge