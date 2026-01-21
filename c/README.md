# LongPort OpenAPI SDK for C

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/c/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

## Examples

Runnable examples live in `examples/c/`:

- `examples/c/account_asset/main.c`
- `examples/c/get_quote/main.c`
- `examples/c/http_client/main.c`
- `examples/c/subscribe_quote/main.c`
- `examples/c/submit_order/main.c`
- `examples/c/today_orders/main.c`

## Quickstart

_Install LongPort OpenAPI SDK_

[`Download C SDK`](https://github.com/longportapp/openapi/releases)

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

```c
#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

void
on_quote(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to get quote: %s\n", lb_error_message(res->error));
    return;
  }

  lb_security_quote_t* data = (lb_security_quote_t*)res->data;
  for (int i = 0; i < res->length; i++) {
    const lb_security_quote_t* quote = &data[i];
    printf("%s timestamp=%ld last_done=%f open=%f high=%f low=%f volume=%ld "
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

void
on_quote_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create quote context: %s\n",
           lb_error_message(res->error));
    return;
  }

  *((const lb_quote_context_t**)res->userdata) = res->ctx;

  const char* symbols[] = { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" };
  lb_quote_context_quote(res->ctx, symbols, 4, on_quote, NULL);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;
  lb_config_t* config = lb_config_from_env(&err);
  if (err) {
    printf("failed to load configuration from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  const lb_quote_context_t* ctx = NULL;
  lb_quote_context_new(config, on_quote_context_created, &ctx);
  getchar();
  lb_quote_context_release(ctx);
  return 0;
}
```

## Quote API _(Subscribe quotes)_

```c
#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

void
on_quote(const struct lb_quote_context_t* ctx,
         const struct lb_push_quote_t* quote,
         void* userdata)
{
  printf("%s timestamp=%ld last_done=%f open=%f high=%f low=%f volume=%ld "
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

void
on_subscrbe(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to subscribe quote: %s\n", lb_error_message(res->error));
    return;
  }
}

void
on_quote_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create quote context: %s\n",
           lb_error_message(res->error));
    return;
  }

  *((const lb_quote_context_t**)res->userdata) = res->ctx;
  lb_quote_context_set_on_quote(res->ctx, on_quote, NULL, NULL);

  const char* symbols[] = { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" };
  lb_quote_context_subscribe(
    res->ctx, symbols, 4, LB_SUBFLAGS_QUOTE, on_subscrbe, NULL);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;
  lb_config_t* config = lb_config_from_env(&err);
  if (err) {
    printf("failed to load configuration from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  const lb_quote_context_t* ctx = NULL;
  lb_quote_context_new(config, on_quote_context_created, &ctx);
  getchar();
  lb_quote_context_release(ctx);
  return 0;
}
```

## Trade API _(Submit order)_

```c
#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

void
on_submit_order(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to submit order: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_submit_order_response_t* resp = res->data;
  printf("order id: %s\n", resp->order_id);
}

void
on_trade_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create trade context: %s\n",
           lb_error_message(res->error));
    return;
  }

  *((const lb_quote_context_t**)res->userdata) = res->ctx;

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

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;
  lb_config_t* config = lb_config_from_env(&err);
  if (err) {
    printf("failed to load configuration from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  const lb_trade_context_t* ctx = NULL;
  lb_trade_context_new(config, on_trade_context_created, (void*)&ctx);
  getchar();
  lb_trade_context_release(ctx);
  return 0;
}
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If you don't see callbacks, keep the process alive (examples use `getchar()`).
- If building on Linux/macOS, ensure `ncurses` is installed (examples link it on non-Windows).
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
