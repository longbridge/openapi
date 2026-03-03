#include <longport.h>
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

void
on_oauth_refresh(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("refresh failed, re-authorizing: %s\n",
           lb_error_message(res->error));
    lb_oauth_t* oauth = lb_oauth_new(CLIENT_ID);
    lb_oauth_authorize(oauth, on_open_url, NULL, on_oauth_authorize,
                       res->userdata);
    lb_oauth_free(oauth);
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
  lb_oauth_t* oauth = lb_oauth_new(CLIENT_ID);

  if (token && !lb_oauth_token_is_expired(token) &&
      !lb_oauth_token_expires_soon(token)) {
    proceed(token, &ctx);
    lb_oauth_token_free(token);
  } else if (token && lb_oauth_token_expires_soon(token)) {
    lb_oauth_refresh(oauth, token, on_oauth_refresh, &ctx);
    lb_oauth_token_free(token);
  } else {
    if (token) lb_oauth_token_free(token);
    lb_error_free(load_err);
    lb_oauth_authorize(oauth, on_open_url, NULL, on_oauth_authorize, &ctx);
  }

  lb_oauth_free(oauth);

  getchar();
  lb_trade_context_release(ctx);
  return 0;
}
