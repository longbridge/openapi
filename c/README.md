# Longbridge OpenAPI SDK for C

`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).

## Documentation

- SDK docs: https://longbridge.github.io/openapi/c/index.html
- Longbridge OpenAPI: https://open.longbridge.com/en/

## Examples

Runnable examples live in `examples/c/`:

- `examples/c/account_asset/main.c`
- `examples/c/get_quote/main.c`
- `examples/c/http_client/main.c`
- `examples/c/subscribe_quote/main.c`
- `examples/c/submit_order/main.c`
- `examples/c/today_orders/main.c`

## Quickstart

_Install Longbridge OpenAPI SDK_

[`Download C SDK`](https://github.com/longbridge/openapi/releases)

### Authentication

Longbridge OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

_bash / macOS / Linux_

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

_PowerShell (Windows)_

```powershell
Invoke-RestMethod -Method Post -Uri https://openapi.longbridgeapp.com/oauth2/register `
  -ContentType "application/json" `
  -Body '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "client_name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Build OAuth client and create a Config**

`lb_oauth_new` loads a cached token from
`~/.longbridge-openapi/tokens/<client_id>` (`%USERPROFILE%\.longbridge-openapi\tokens\<client_id>` on Windows)
if one exists and is still valid, or starts the browser authorization flow
automatically.  The token is persisted to the same path after a successful
authorization or refresh.  The resulting `lb_oauth_t*` handle is passed
directly to `lb_config_from_oauth`.

```c
#include <longbridge.h>
#include <stdio.h>

static void
on_open_url(const char* url, void* userdata)
{
  printf("Open this URL to authorize: %s\n", url);
}

static void
on_oauth_ready(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("OAuth failed: %s\n", lb_error_message(res->error));
    return;
  }

  lb_oauth_t* oauth = (lb_oauth_t*)res->data;
  lb_config_t* config = lb_config_from_oauth(oauth);
  // Use config to create contexts...
  lb_config_free(config);
  lb_oauth_free(oauth);
}

int
main(int argc, char const* argv[])
{
  lb_oauth_new("your-client-id", 0, on_open_url, NULL, on_oauth_ready, NULL);
  getchar();
  return 0;
}
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```c
#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#endif

static void
on_quote(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to get quote: %s\n", lb_error_message(res->error));
    return;
  }

  lb_security_quote_t* data = (lb_security_quote_t*)res->data;
  for (int i = 0; i < res->length; i++) {
    const lb_security_quote_t* quote = &data[i];
    printf("%s timestamp=%lld last_done=%f open=%f high=%f low=%f volume=%lld "
           "turnover=%f\n",
           quote->symbol,
           quote->timestamp,
           lb_decimal_to_double(quote->last_done),
           lb_decimal_to_double(quote->open),
           lb_decimal_to_double(quote->high),
           lb_decimal_to_double(quote->low),
           quote->volume,
           lb_decimal_to_double(quote->turnover));
  }
}

static void
on_quote_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create quote context: %s\n", lb_error_message(res->error));
    return;
  }

  const char* symbols[] = { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" };
  lb_quote_context_quote(res->ctx, symbols, 4, on_quote, NULL);
}

static void
on_oauth_ready(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("OAuth failed: %s\n", lb_error_message(res->error));
    return;
  }

  lb_oauth_t* oauth = (lb_oauth_t*)res->data;
  lb_config_t* config = lb_config_from_oauth(oauth);
  lb_quote_context_new(config, on_quote_context_created, NULL);
  lb_config_free(config);
  lb_oauth_free(oauth);
}

static void
on_open_url(const char* url, void* userdata)
{
  printf("Open this URL to authorize: %s\n", url);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_oauth_new("your-client-id", 0, on_open_url, NULL, on_oauth_ready, NULL);
  getchar();
  return 0;
}
```

## Quote API _(Subscribe quotes)_

```c
#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#endif

static void
on_quote(const struct lb_quote_context_t* ctx,
         const struct lb_push_quote_t* quote,
         void* userdata)
{
  printf("%s timestamp=%lld last_done=%f open=%f high=%f low=%f volume=%lld "
         "turnover=%f\n",
         quote->symbol,
         quote->timestamp,
         lb_decimal_to_double(quote->last_done),
         lb_decimal_to_double(quote->open),
         lb_decimal_to_double(quote->high),
         lb_decimal_to_double(quote->low),
         quote->volume,
         lb_decimal_to_double(quote->turnover));
}

static void
on_subscribe(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to subscribe: %s\n", lb_error_message(res->error));
  }
}

static void
on_quote_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create quote context: %s\n", lb_error_message(res->error));
    return;
  }

  lb_quote_context_set_on_quote(res->ctx, on_quote, NULL, NULL);

  const char* symbols[] = { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" };
  lb_quote_context_subscribe(res->ctx, symbols, 4, LB_SUBFLAGS_QUOTE, on_subscribe, NULL);
}

static void
on_oauth_ready(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("OAuth failed: %s\n", lb_error_message(res->error));
    return;
  }

  lb_oauth_t* oauth = (lb_oauth_t*)res->data;
  lb_config_t* config = lb_config_from_oauth(oauth);
  lb_quote_context_new(config, on_quote_context_created, NULL);
  lb_config_free(config);
  lb_oauth_free(oauth);
}

static void
on_open_url(const char* url, void* userdata)
{
  printf("Open this URL to authorize: %s\n", url);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_oauth_new("your-client-id", 0, on_open_url, NULL, on_oauth_ready, NULL);
  getchar();
  return 0;
}
```

## Trade API _(Submit order)_

```c
#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#endif

static void
on_submit_order(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to submit order: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_submit_order_response_t* resp = res->data;
  printf("order id: %s\n", resp->order_id);
}

static void
on_trade_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create trade context: %s\n", lb_error_message(res->error));
    return;
  }

  lb_decimal_t* submitted_price = lb_decimal_from_double(50.0);
  lb_decimal_t* submitted_quantity = lb_decimal_from_double(200.0);
  lb_submit_order_options_t opts = {
    "700.HK",       OrderTypeLO,
    OrderSideBuy,   submitted_quantity,
    TimeInForceDay, submitted_price,
    NULL,           NULL,
    NULL,           NULL,
    NULL,           NULL,
    NULL,
  };
  lb_decimal_free(submitted_price);
  lb_decimal_free(submitted_quantity);
  lb_trade_context_submit_order(res->ctx, &opts, on_submit_order, NULL);
}

static void
on_oauth_ready(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("OAuth failed: %s\n", lb_error_message(res->error));
    return;
  }

  lb_oauth_t* oauth = (lb_oauth_t*)res->data;
  lb_config_t* config = lb_config_from_oauth(oauth);
  lb_trade_context_new(config, on_trade_context_created, NULL);
  lb_config_free(config);
  lb_oauth_free(oauth);
}

static void
on_open_url(const char* url, void* userdata)
{
  printf("Open this URL to authorize: %s\n", url);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_oauth_new("your-client-id", 0, on_open_url, NULL, on_oauth_ready, NULL);
  getchar();
  return 0;
}
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If you don't see callbacks, keep the process alive (examples use `getchar()`).
- If building on Linux/macOS, ensure `ncurses` is installed (examples link it on non-Windows).
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
