#pragma once

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
  /// @param app_key       App key
  /// @param app_secret    App secret
  /// @param access_token  Access token
  static Config from_apikey(const std::string& app_key,
                            const std::string& app_secret,
                            const std::string& access_token);

  /// Create a new `Config` from environment variables (API Key mode)
  ///
  /// Variables: `LONGPORT_APP_KEY`, `LONGPORT_APP_SECRET`,
  /// `LONGPORT_ACCESS_TOKEN`, `LONGPORT_HTTP_URL`, `LONGPORT_QUOTE_WS_URL`,
  /// `LONGPORT_TRADE_WS_URL`, `LONGPORT_LANGUAGE`, `LONGPORT_ENABLE_OVERNIGHT`,
  /// `LONGPORT_PUSH_CANDLESTICK_MODE`, `LONGPORT_PRINT_QUOTE_PACKAGES`,
  /// `LONGPORT_LOG_PATH`
  static Config from_apikey_env(Status& status);

  /// Create a new `Config` for OAuth 2.0 authentication
  ///
  /// @param oauth OAuth 2.0 client obtained from `OAuthBuilder::build`
  static Config from_oauth(const OAuth& oauth);

  // ── Chainable setters ──────────────────────────────────────────────────────

  /// Set the HTTP endpoint URL
  Config& set_http_url(const std::string& url);

  /// Set the Quote WebSocket endpoint URL
  Config& set_quote_ws_url(const std::string& url);

  /// Set the Trade WebSocket endpoint URL
  Config& set_trade_ws_url(const std::string& url);

  /// Set the language identifier
  Config& set_language(Language language);

  /// Enable overnight quote
  Config& enable_overnight();

  /// Set the push candlestick mode
  Config& set_push_candlestick_mode(PushCandlestickMode mode);

  /// Disable printing of quote packages on connection
  Config& disable_print_quote_packages();

  /// Set the log file path
  Config& set_log_path(const std::string& path);
};

} // namespace longport
