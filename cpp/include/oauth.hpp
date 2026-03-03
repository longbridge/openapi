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

  /// Load a token from the default path (`~/.longbridge-openapi/token`)
  ///
  /// @param out_token  On success, receives the loaded token.
  /// @return Status indicating success or failure
  static Status load(OAuthToken& out_token);

  /// Load a token from an explicit file path
  ///
  /// @param path       Path to the token JSON file
  /// @param out_token  On success, receives the loaded token.
  /// @return Status indicating success or failure
  static Status load_from_path(const std::string& path, OAuthToken& out_token);

  /// Save the token to the default path (`~/.longbridge-openapi/token`)
  ///
  /// The parent directory is created automatically if it does not exist.
  ///
  /// @return Status indicating success or failure
  Status save() const;

  /// Save the token to an explicit file path
  ///
  /// The parent directory is created automatically if it does not exist.
  ///
  /// @param path  Destination path for the token JSON file
  /// @return Status indicating success or failure
  Status save_to_path(const std::string& path) const;
};

/// OAuth 2.0 client for LongPort OpenAPI
class OAuth
{
private:
  lb_oauth_t* oauth_;

public:
  /// Create a new OAuth 2.0 client with the default callback port (60355)
  ///
  /// @param client_id  OAuth 2.0 client ID from the LongPort developer portal
  OAuth(const std::string& client_id);

  OAuth(const OAuth&) = delete;
  OAuth(OAuth&& other);
  ~OAuth();

  /// Set the callback port
  ///
  /// @param callback_port  TCP port for the local callback server (default
  ///                       60355). Must match one of the redirect URIs
  ///                       registered for the client.
  void set_callback_port(uint16_t callback_port);

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
