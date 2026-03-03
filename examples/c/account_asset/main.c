#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

void
on_account_balance(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to get account balance: %s\n", lb_error_message(res->error));
    return;
  }

  lb_account_balance_t* data = (lb_account_balance_t*)res->data;
  for (int i = 0; i < res->length; i++) {
    const lb_account_balance_t* balance = &data[i];

    printf("total_cash: %f\n", lb_decimal_to_double(balance->total_cash));
    printf("max_finance_amount: %f\n",
           lb_decimal_to_double(balance->max_finance_amount));
    printf("remaining_finance_amount: %f\n",
           lb_decimal_to_double(balance->remaining_finance_amount));
    printf("risk_level: %d\n", balance->risk_level);
    printf("margin_call: %f\n", lb_decimal_to_double(balance->margin_call));
    printf("currency: %s\n", balance->currency);
    printf("init_margin: %f\n", lb_decimal_to_double(balance->init_margin));
    printf("maintenance_margin: %f\n",
           lb_decimal_to_double(balance->maintenance_margin));
    printf("cash_infos:\n");

    for (int j = 0; j < data->num_cash_infos; j++) {
      const lb_cash_info_t* cash_info = &data->cash_infos[j];

      printf("\t%s\n", cash_info->currency);
      printf("\t\twithdraw_cash: %f\n",
             lb_decimal_to_double(cash_info->withdraw_cash));
      printf("\t\tavailable_cash: %f\n",
             lb_decimal_to_double(cash_info->available_cash));
      printf("\t\tfrozen_cash: %f\n",
             lb_decimal_to_double(cash_info->frozen_cash));
      printf("\t\tsettling_cash: %f\n",
             lb_decimal_to_double(cash_info->settling_cash));
    }
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
  lb_trade_context_account_balance(res->ctx, NULL, on_account_balance, NULL);
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
