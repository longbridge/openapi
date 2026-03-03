#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

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

void
proceed(const lb_oauth_token_t* token, const lb_quote_context_t** ctx)
{
  lb_config_t* config = lb_config_from_oauth(token);
  lb_quote_context_new(config, on_quote_context_created, ctx);
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

  proceed(token, (const lb_quote_context_t**)res->userdata);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const lb_quote_context_t* ctx = NULL;

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
  lb_quote_context_release(ctx);
  return 0;
}
