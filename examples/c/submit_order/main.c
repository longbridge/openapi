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
  lb_submit_order_options_t opts = {
    "700.HK", OrderTypeLO, OrderSideBuy, 200,  TimeInForceDay, submitted_price,
    NULL,     NULL,        NULL,         NULL, NULL,           NULL,
    NULL,
  };
  lb_decimal_free(submitted_price);
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