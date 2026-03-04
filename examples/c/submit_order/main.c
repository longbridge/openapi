#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

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

  *((const lb_trade_context_t**)res->userdata) = res->ctx;

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
  lb_trade_context_submit_order(res->ctx, &opts, on_submit_order, NULL);
}

void
on_open_url(const char* url, void* userdata)
{
  (void)userdata;
  printf("Open this URL to authorize: %s\n", url);
}

void
on_oauth_ready(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("OAuth failed: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_oauth_t* oauth = (const lb_oauth_t*)res->data;
  lb_config_t* config = lb_config_from_oauth(oauth);
  lb_oauth_free((lb_oauth_t*)oauth);
  lb_trade_context_new(config, on_trade_context_created, res->userdata);
  lb_config_free(config);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const lb_trade_context_t* ctx = NULL;

  lb_oauth_new(CLIENT_ID, 0, on_open_url, NULL, on_oauth_ready, &ctx);

  getchar();
  lb_trade_context_release(ctx);
  return 0;
}
