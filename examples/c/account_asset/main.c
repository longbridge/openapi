#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

void
on_account_balance(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to get account balance: %s\n", lb_error_message(res->error));
    return;
  }

  lb_account_balance_t* data = (lb_account_balance_t*)res->data;
  for (int i = 0; i < res->length; i++) {
    printf("total_cash: %f\n", lb_decimal_to_double(data->total_cash));
    printf("max_finance_amount: %f\n",
           lb_decimal_to_double(data->max_finance_amount));
    printf("remaining_finance_amount: %f\n",
           lb_decimal_to_double(data->remaining_finance_amount));
    printf("risk_level: %d\n", data->risk_level);
    printf("margin_call: %f\n", lb_decimal_to_double(data->margin_call));
    printf("currency: %s\n", data->currency);
    printf("init_margin: %f\n", lb_decimal_to_double(data->init_margin));
    printf("maintenance_margin: %f\n",
           lb_decimal_to_double(data->maintenance_margin));
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

  *((const lb_quote_context_t**)res->userdata) = res->ctx;
  lb_trade_context_account_balance(res->ctx, on_account_balance, NULL);
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
  lb_trade_context_new(config, on_trade_context_created, &ctx);
  getchar();
  lb_trade_context_release(ctx);
  return 0;
}