# LongPort OpenAPI SDK for C++

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/cpp/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

## Examples

Runnable examples live in `examples/cpp/`:

- `examples/cpp/get_quote/main.cpp`
- `examples/cpp/history_candlesticks_by_offset/main.cpp`
- `examples/cpp/http_client/main.cpp`
- `examples/cpp/subscribe_candlesticks/main.cpp`
- `examples/cpp/subscribe_quote/main.cpp`
- `examples/cpp/submit_order/main.cpp`
- `examples/cpp/today_orders/main.cpp`

## Quickstart

_Install LongPort OpenAPI SDK_

[`Download C++ SDK`](https://github.com/longportapp/openapi/releases)

### Authentication

LongPort OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Authorize, Refresh, and Get Token**

```c++
#include <iostream>
#include <longport.hpp>

using namespace longport;

static void
run(const OAuthToken& token)
{
  Config config = Config::from_oauth(token);
  // Use config to create contexts...
}

int
main(int argc, char const* argv[])
{
  OAuthToken token;
  Status load_status = OAuthToken::load(token);
  if (load_status) {
    if (token.is_expired()) {
      // Token has expired — re-authorize
      OAuth oauth("your-client-id");
      oauth.authorize(
        [](const std::string& url) {
          std::cout << "Open this URL to authorize: " << url << std::endl;
        },
        [](auto res) {
          if (!res) {
            std::cout << "authorization failed: " << *res.status().message()
                      << std::endl;
            return;
          }
          res->save();
          run(*res);
        });
    } else if (token.expires_soon()) {
      // Token will expire soon — refresh it
      OAuth oauth("your-client-id");
      oauth.refresh(token, [](auto res) {
        if (!res) {
          std::cout << "refresh failed: " << *res.status().message()
                    << std::endl;
          return;
        }
        res->save();
        run(*res);
      });
    } else {
      run(token);
    }
  } else {
    // No saved token — start authorization flow
    OAuth oauth("your-client-id");
    oauth.authorize(
      [](const std::string& url) {
        std::cout << "Open this URL to authorize: " << url << std::endl;
      },
      [](auto res) {
        if (!res) {
          std::cout << "authorization failed: " << *res.status().message()
                    << std::endl;
          return;
        }
        res->save();
        run(*res);
      });
  }

  std::cin.get();
  return 0;
}
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::quote;

static void
run(const OAuthToken& token)
{
  Config config = Config::from_oauth(token);

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: "
                << *res.status().message() << std::endl;
      return;
    }

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };
    res.context().quote(symbols, [](auto res) {
      if (!res) {
        std::cout << "failed to get quote: " << *res.status().message()
                  << std::endl;
        return;
      }

      for (auto it = res->cbegin(); it != res->cend(); ++it) {
        std::cout << it->symbol << " timestamp=" << it->timestamp
                  << " last_done=" << (double)it->last_done
                  << " prev_close=" << (double)it->prev_close
                  << " open=" << (double)it->open
                  << " high=" << (double)it->high
                  << " low=" << (double)it->low
                  << " volume=" << it->volume
                  << " turnover=" << it->turnover << std::endl;
      }
    });
  });
}

static void
authorize_and_run(const std::string& client_id)
{
  OAuth oauth(client_id);
  oauth.authorize(
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }
      res->save();
      run(*res);
    });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const std::string client_id = "your-client-id";
  OAuthToken token;
  Status load_status = OAuthToken::load(token);
  if (load_status) {
    if (token.is_expired()) {
      authorize_and_run(client_id);
    } else if (token.expires_soon()) {
      OAuth oauth(client_id);
      oauth.refresh(token, [&client_id](auto res) {
        if (!res) {
          std::cout << "refresh failed: " << *res.status().message()
                    << std::endl;
          authorize_and_run(client_id);
          return;
        }
        res->save();
        run(*res);
      });
    } else {
      run(token);
    }
  } else {
    authorize_and_run(client_id);
  }

  std::cin.get();
  return 0;
}
```

## Quote API _(Subscribe quotes)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::quote;

static void
run(const OAuthToken& token)
{
  Config config = Config::from_oauth(token);

  QuoteContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create quote context: "
                << *res.status().message() << std::endl;
      return;
    }

    res.context().set_on_quote([](auto event) {
      std::cout << event->symbol << " timestamp=" << event->timestamp
                << " last_done=" << (double)event->last_done
                << " open=" << (double)event->open
                << " high=" << (double)event->high
                << " low=" << (double)event->low
                << " volume=" << event->volume
                << " turnover=" << event->turnover << std::endl;
    });

    std::vector<std::string> symbols = {
      "700.HK", "AAPL.US", "TSLA.US", "NFLX.US"
    };

    res.context().subscribe(symbols, SubFlags::QUOTE(), [](auto res) {
      if (!res) {
        std::cout << "failed to subscribe quote: "
                  << *res.status().message() << std::endl;
        return;
      }
    });
  });
}

static void
authorize_and_run(const std::string& client_id)
{
  OAuth oauth(client_id);
  oauth.authorize(
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }
      res->save();
      run(*res);
    });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const std::string client_id = "your-client-id";
  OAuthToken token;
  Status load_status = OAuthToken::load(token);
  if (load_status) {
    if (token.is_expired()) {
      authorize_and_run(client_id);
    } else if (token.expires_soon()) {
      OAuth oauth(client_id);
      oauth.refresh(token, [&client_id](auto res) {
        if (!res) {
          std::cout << "refresh failed: " << *res.status().message()
                    << std::endl;
          authorize_and_run(client_id);
          return;
        }
        res->save();
        run(*res);
      });
    } else {
      run(token);
    }
  } else {
    authorize_and_run(client_id);
  }

  std::cin.get();
  return 0;
}
```

## Trade API _(Submit order)_

```c++
#include <iostream>
#include <longport.hpp>

#ifdef WIN32
#include <windows.h>
#endif

using namespace longport;
using namespace longport::trade;

static void
run(const OAuthToken& token)
{
  Config config = Config::from_oauth(token);

  TradeContext::create(config, [](auto res) {
    if (!res) {
      std::cout << "failed to create trade context: "
                << *res.status().message() << std::endl;
      return;
    }

    SubmitOrderOptions opts{
      "700.HK",     OrderType::LO,        OrderSide::Buy,
      Decimal(200), TimeInForceType::Day, Decimal(50.0),
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt, std::nullopt,         std::nullopt,
      std::nullopt,
    };
    res.context().submit_order(opts, [](auto res) {
      if (!res) {
        std::cout << "failed to submit order: " << *res.status().message()
                  << std::endl;
        return;
      }
      std::cout << "order id: " << res->order_id << std::endl;
    });
  });
}

static void
authorize_and_run(const std::string& client_id)
{
  OAuth oauth(client_id);
  oauth.authorize(
    [](const std::string& url) {
      std::cout << "Open this URL to authorize: " << url << std::endl;
    },
    [](auto res) {
      if (!res) {
        std::cout << "authorization failed: " << *res.status().message()
                  << std::endl;
        return;
      }
      res->save();
      run(*res);
    });
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const std::string client_id = "your-client-id";
  OAuthToken token;
  Status load_status = OAuthToken::load(token);
  if (load_status) {
    if (token.is_expired()) {
      authorize_and_run(client_id);
    } else if (token.expires_soon()) {
      OAuth oauth(client_id);
      oauth.refresh(token, [&client_id](auto res) {
        if (!res) {
          std::cout << "refresh failed: " << *res.status().message()
                    << std::endl;
          authorize_and_run(client_id);
          return;
        }
        res->save();
        run(*res);
      });
    } else {
      run(token);
    }
  } else {
    authorize_and_run(client_id);
  }

  std::cin.get();
  return 0;
}
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If you don't see push events, keep the process alive (examples use `std::cin.get()`).
- If building on Linux/macOS, ensure `ncurses` is installed (examples link it on non-Windows).
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
