#pragma once

#include <cstdint>
#include <functional>
#include <string>

#include "async_result.hpp"
#include "status.hpp"

typedef struct lb_oauth_t lb_oauth_t;
typedef struct lb_oauth_token_t lb_oauth_token_t;

namespace longport {

/// OAuth 2.0 access token
///
/// Owns the underlying `lb_oauth_token_t*`; freed on destruction.
class OAuthToken
{
private:
  lb_oauth_token_t* token_;

public:
  explicit OAuthToken(lb_oauth_token_t* token) : token_(token) {}

  OAuthToken(const OAuthToken&) = delete;
  OAuthToken& operator=(const OAuthToken&) = delete;

  OAuthToken(OAuthToken&& other) : token_(other.token_) { other.token_ = nullptr; }
  OAuthToken& operator=(OAuthToken&& other);

  ~OAuthToken();

  /// The underlying C token pointer (non-owning)
  const lb_oauth_token_t* get() const { return token_; }

  /// Returns true if the token has expired
  bool is_expired() const;

  /// Returns true if the token will expire within 1 hour
  bool expires_soon() const;
};

/// OAuth 2.0 client for LongPort OpenAPI
class OAuth
{
private:
  lb_oauth_t* oauth_;

public:
  /// Create a new OAuth 2.0 client
  ///
  /// @param client_id  OAuth 2.0 client ID from the LongPort developer portal
  OAuth(const std::string& client_id);

  OAuth(const OAuth&) = delete;
  OAuth(OAuth&& other);
  ~OAuth();

  /// Start the OAuth 2.0 authorization flow (async)
  ///
  /// The `open_url` callback is invoked with the authorization URL so the
  /// caller can open it in a browser or handle it in any other way.
  ///
  /// @param open_url  Called with the authorization URL
  /// @param callback  Invoked on completion; result data is `OAuthToken*`
  void authorize(std::function<void(const std::string&)> open_url,
                 AsyncCallback<void*, OAuthToken> callback);

  /// Refresh an access token (async)
  ///
  /// @param token     Existing token whose refresh_token field is used
  /// @param callback  Invoked on completion; result data is `OAuthToken*`
  void refresh(const OAuthToken& token, AsyncCallback<void*, OAuthToken> callback);
};

} // namespace longport
