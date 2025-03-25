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
    printf("%s timestamp=%lld last_done=%f open=%f high=%f low=%f volume=%lld "
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