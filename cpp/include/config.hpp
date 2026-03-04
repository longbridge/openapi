#pragma once

#include <optional>
#include <string>
#include <utility>

#include "async_result.hpp"
#include "oauth.hpp"
#include "status.hpp"
#include "types.hpp"

typedef struct lb_config_t lb_config_t;

namespace longport {

class Config
{
private:
  lb_config_t* config_;

public:
  Config();
  Config(lb_config_t* config);
  Config(const Config&) = delete;
  Config(Config&& other);

  ~Config();

  operator const lb_config_t*() const;

  /// Create a new `Config` for API Key authentication
  ///
  /// @param app_key   App key
  /// @param app_secret App secret
  /// @param access_token Access token
  /// @param http_url HTTP endpoint url (Default:
  /// https://openapi.portbridgeapp.com)
  /// @param quote_ws_url Quote websocket endpoint url (Default:
  /// wss://openapi-quote.longportapp.com/v2)
  /// @param trade_ws_url Trade websocket endpoint url (Default:
  /// wss://openapi-trade.longportapp.com/v2)
  /// @param language Language identifer (Default: Language::EN)
  /// @param enable_overnight Enable overnight quote (Default: false)
  /// @param push_candlestick_mode Push candlestick mode (Default:
  /// PushCandlestickMode::Realtime)
  /// @param enable_print_quote_packages Print quote packages when connected
  /// (Default: true)
  /// @param log_path Path for log files (Default: no logs)
  static Config from_apikey(
    const std::string& app_key,
    const std::string& app_secret,
    const std::string& access_token,
    const std::optional<std::string>& http_url = std::nullopt,
    const std::optional<std::string>& quote_ws_url = std::nullopt,
    const std::optional<std::string>& trade_ws_url = std::nullopt,
    const std::optional<Language>& language = std::nullopt,
    bool enable_overnight = false,
    const std::optional<PushCandlestickMode>& push_candlestick_mode =
      std::nullopt,
    bool enable_print_quote_packages = true,
    const std::optional<std::string>& log_path = std::nullopt);

  /// Create a new `Config` from environment variables (API Key mode)
  ///
  /// It first gets the environment variables from the `.env` file in the
  /// current directory.
  ///
  /// # Variables
  ///
  /// - `LONGPORT_LANGUAGE` - Language identifier, `zh-CN`, `zh-HK` or `en`
  ///   (Default: `en`)
  /// - `LONGPORT_APP_KEY` - App key
  /// - `LONGPORT_APP_SECRET` - App secret
  /// - `LONGPORT_ACCESS_TOKEN` - Access token
  /// - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default:
  /// `https://openapi.longportapp.com`)
  /// - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
  ///   `wss://openapi-quote.longportapp.com/v2`)
  /// - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
  ///   `wss://openapi-trade.longportapp.com/v2`)
  /// - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or
  ///   `false` (Default: `false`)
  /// - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
  ///   `realtime`)
  /// - `LONGPORT_PRINT_QUOTE_PACKAGES` - Print quote packages when connected,
  ///   `true` or `false` (Default: `true`)
  /// - `LONGPORT_LOG_PATH` - Set the path of the log files (Default: `no logs`)
  static Config from_apikey_env(Status& status);

  /// Create a new `Config` for OAuth 2.0 authentication
  ///
  /// OAuth 2.0 is the recommended authentication method that uses Bearer tokens
  /// and does not require app_secret or HMAC signatures.
  ///
  /// @param oauth OAuth 2.0 client obtained from `OAuthBuilder::build`
  static Config from_oauth(const OAuth& oauth);
};

} // namespace longport
