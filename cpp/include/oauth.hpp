#pragma once

#include <cstdint>
#include <functional>
#include <string>

#include "async_result.hpp"
#include "status.hpp"

typedef struct lb_oauth_t lb_oauth_t;

namespace longbridge {

/// OAuth 2.0 client (opaque handle)
///
/// Copyable: copy increments the internal Arc reference count via
/// `lb_oauth_clone`. Always freed on destruction via `lb_oauth_free`.
class OAuth
{
private:
  lb_oauth_t* oauth_;

public:
  explicit OAuth(lb_oauth_t* oauth);
  OAuth(const OAuth& other);
  OAuth(OAuth&& other);
  OAuth& operator=(const OAuth& other);
  OAuth& operator=(OAuth&& other);
  ~OAuth();

  operator const lb_oauth_t*() const { return oauth_; }

  /// Asynchronously revoke the current token on the server (RFC 7009)
  ///
  /// The refresh token is revoked when present, otherwise the access token.
  /// Only the server is contacted; the cached token on disk is left untouched
  /// (see `clear`).
  ///
  /// @param callback  Invoked on completion; check `status()` for errors
  void revoke(AsyncCallback<NoContext, void> callback) const;

  /// Asynchronously clear the token locally
  ///
  /// Drops the in-memory token and deletes the cached token file
  /// (`~/.longbridge/openapi/tokens/<client_id>`). The server is not contacted
  /// (see `revoke`).
  ///
  /// @param callback  Invoked on completion; check `status()` for errors
  void clear(AsyncCallback<NoContext, void> callback) const;
};

/// Builder for constructing an OAuth 2.0 client
///
/// Tries to load an existing token from
/// `~/.longbridge/openapi/tokens/<client_id>`. If the token is missing or
/// expired, starts a local callback server and calls `open_url` so the caller
/// can open the authorization URL in a browser.
class OAuthBuilder
{
private:
  std::string client_id_;
  uint16_t callback_port_;

public:
  /// @param client_id     OAuth 2.0 client ID from the Longbridge developer portal
  /// @param callback_port Local callback server port; pass 0 to use the
  ///                      default (60355)
  OAuthBuilder(const std::string& client_id, uint16_t callback_port = 0);

  /// Asynchronously build an OAuth 2.0 client
  ///
  /// @param open_url  Called with the authorization URL during the auth flow
  /// @param callback  Invoked on completion; result data is `OAuth*`
  void build(std::function<void(const std::string&)> open_url,
             AsyncCallback<NoContext, OAuth> callback);
};

} // namespace longbridge
