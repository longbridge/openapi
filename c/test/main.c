#include <longport.h>
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
    printf("init_margin: %f\n", lb_decimal_to_double(data->init_margin));
    for (int j = 0; j < data->num_cash_infos; j++) {
      printf("currency: %s\n", data->cash_infos[j].currency);
      printf("available_cash: %f\n",
             lb_decimal_to_double(data->cash_infos[j].available_cash));
    }
  }
}

void
on_trade_context_created(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to create quote context: %s\n",
           lb_error_message(res->error));
    return;
  }

  lb_trade_context_account_balance(res->ctx, NULL, on_account_balance, NULL);
  lb_quote_context_release(res->ctx);
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

  lb_trade_context_new(config, on_trade_context_created, NULL);
  getchar();
  return 0;
}