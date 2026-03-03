#include <longport.h>
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
proceed(const lb_oauth_token_t* token, const lb_trade_context_t** ctx)
{
  lb_config_t* config = lb_config_from_oauth(token);
  lb_trade_context_new(config, on_trade_context_created, ctx);
  lb_config_free(config);
}

void
on_open_url(const char* url, void* userdata)
{
  printf("Open this URL to authorize: %s\n", url);
}

void
on_oauth_authorize(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("authorization failed: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_oauth_token_t* token = (const lb_oauth_token_t*)res->data;

  lb_error_t* save_err = NULL;
  lb_oauth_token_save(token, &save_err);
  if (save_err) {
    printf("failed to save token: %s\n", lb_error_message(save_err));
    lb_error_free(save_err);
  }

  proceed(token, (const lb_trade_context_t**)res->userdata);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const lb_trade_context_t* ctx = NULL;

  lb_error_t* load_err = NULL;
  lb_oauth_token_t* token = lb_oauth_token_load(&load_err);
  if (token) {
    proceed(token, &ctx);
    lb_oauth_token_free(token);
  } else {
    lb_error_free(load_err);
    lb_oauth_t* oauth = lb_oauth_new(CLIENT_ID);
    lb_oauth_authorize(oauth, on_open_url, NULL, on_oauth_authorize, &ctx);
    lb_oauth_free(oauth);
  }

  getchar();
  lb_trade_context_release(ctx);
  return 0;
}
