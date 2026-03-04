#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

void
on_today_orders(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to get today orders: %s\n", lb_error_message(res->error));
    return;
  }

  lb_order_t* data = (lb_order_t*)res->data;
  for (int i = 0; i < res->length; i++) {
    const lb_order_t* order = &data[i];
    printf("order_id=%s status=%d symbol=%s stock_name=%s order_type=%d\n",
           order->order_id,
           order->status,
           order->symbol,
           order->stock_name,
           order->order_type);
  }
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
  lb_trade_context_today_orders(res->ctx, NULL, on_today_orders, NULL);
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
