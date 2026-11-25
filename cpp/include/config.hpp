#pragma once

#include <string>
#include <utility>

#include "async_result.hpp"
#include "oauth.hpp"
#include "status.hpp"
#include "types.hpp"

typedef struct lb_config_t lb_config_t;

namespace longbridge {

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
  /// Optional environment variables are read automatically:
  /// `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
  /// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
  /// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_QUOTE_TRANSPORT`,
  /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
  /// `LONGBRIDGE_LOG_PATH`.  Use the chainable `set_*` methods to override any
  /// of these values.
  ///
  /// @param app_key       App key
  /// @param app_secret    App secret
  /// @param access_token  Access token
  static Config from_apikey(const std::string& app_key,
                            const std::string& app_secret,
                            const std::string& access_token);

  /// Create a new `Config` from environment variables (API Key mode)
  ///
  /// Variables: `LONGBRIDGE_APP_KEY`, `LONGBRIDGE_APP_SECRET`,
  /// `LONGBRIDGE_ACCESS_TOKEN`, `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_QUOTE_WS_URL`,
  /// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
  /// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_QUOTE_TRANSPORT`,
  /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
  /// `LONGBRIDGE_LOG_PATH`
  static Config from_apikey_env(Status& status);

  /// Create a new `Config` for OAuth 2.0 authentication
  ///
  /// Optional environment variables are read automatically:
  /// `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
  /// `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
  /// `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_QUOTE_TRANSPORT`,
  /// `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
  /// `LONGBRIDGE_LOG_PATH`.  Use the chainable `set_*` methods to override any
  /// of these values.
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

  /// Enable paper trading mode.
  ///
  /// Every request then carries `x-papertrading: true`, which asks the server
  /// to serve the paper trading (simulation) account and to reject a token
  /// that belongs to a real-money account.
  ///
  /// This is a server-side check, not a client-side guarantee. Two limits
  /// apply today, and neither is visible in the response:
  ///
  /// - The US data center (LBUS) has no paper trading environment. There is no
  ///   paper trading account to serve, and the server answers with the
  ///   real-money account instead of rejecting the request. The SDK therefore
  ///   refuses order submission, replacement and multi-leg submission
  ///   locally; reads are not refused and return real-money data.
  /// - Under OAuth authentication the server does not apply the check at all,
  ///   in any region: a real-money token is accepted and reads return
  ///   real-money data without an error.
  ///
  /// Do not rely on this option as the only thing standing between your
  /// strategy and your real-money account.
  ///
  /// By default this option is disabled and the server imposes no restrictions
  /// — both paper trading and real-money accounts are accepted.
  Config& enable_papertrading();

  /// Set the push candlestick mode
  Config& set_push_candlestick_mode(PushCandlestickMode mode);

  /// Set the transport used by the quote pull APIs
  ///
  /// Default: `QuoteTransport::WebSocket` (or `LONGBRIDGE_QUOTE_TRANSPORT` =
  /// `ws` / `http`)
  Config& set_quote_transport(QuoteTransport transport);

  /// Disable printing of quote packages on connection
  Config& disable_print_quote_packages();

  /// Set the log file path
  Config& set_log_path(const std::string& path);

  /// Gets a new `access_token`
  ///
  /// This method is only available when using **Legacy API Key**
  /// authentication (i.e. `Config::from_apikey`). It is not supported for
  /// OAuth 2.0 mode.
  ///
  /// @param expired_at  Unix timestamp for token expiry. Pass `0` to use the
  ///                    default (90 days from now).
  /// @param callback    Callback invoked with the new access token string.
  void refresh_access_token(int64_t expired_at,
                            AsyncCallback<NoContext, std::string> callback);
};

} // namespace longbridge
