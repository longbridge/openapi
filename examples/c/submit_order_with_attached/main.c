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
  const lb_trade_context_t* ctx = lb_trade_context_new(config);
  lb_config_free(config);

  if (!ctx) {
    printf("failed to create trade context\n");
    return;
  }

  *((const lb_trade_context_t**)res->userdata) = ctx;

  lb_decimal_t* submitted_quantity = lb_decimal_from_str("1");
  lb_decimal_t* submitted_price    = lb_decimal_from_str("200");
  lb_decimal_t* profit_taker_price = lb_decimal_from_str("220");
  lb_decimal_t* stop_loss_price    = lb_decimal_from_str("180");

  CSubmitAttachedParams attached = {
    .attached_order_type = AttachedOrderTypeBracket,
    .profit_taker_price  = profit_taker_price,
    .stop_loss_price     = stop_loss_price,
  };

  lb_submit_order_options_t opts = {
    .symbol             = "AAPL.US",
    .order_type         = OrderTypeLO,
    .side               = OrderSideBuy,
    .submitted_quantity = submitted_quantity,
    .time_in_force      = TimeInForceDay,
    .submitted_price    = submitted_price,
    .attached_params    = &attached,
  };

  lb_trade_context_submit_order(ctx, &opts, on_submit_order, NULL);

  lb_decimal_free(submitted_quantity);
  lb_decimal_free(submitted_price);
  lb_decimal_free(profit_taker_price);
  lb_decimal_free(stop_loss_price);
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
