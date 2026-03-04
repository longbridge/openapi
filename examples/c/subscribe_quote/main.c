#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

void
on_quote(const struct lb_quote_context_t* ctx,
         const struct lb_push_quote_t* quote,
         void* userdata)
{
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

void
on_subscribe(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed to subscribe quote: %s\n", lb_error_message(res->error));
    return;
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
  lb_quote_context_set_on_quote(res->ctx, on_quote, NULL, NULL);

  const char* symbols[] = { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" };
  lb_quote_context_subscribe(
    res->ctx, symbols, 4, LB_SUBFLAGS_QUOTE, on_subscribe, NULL);
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
  lb_quote_context_new(config, on_quote_context_created, res->userdata);
  lb_config_free(config);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  const lb_quote_context_t* ctx = NULL;

  lb_oauth_new(CLIENT_ID, 0, on_open_url, NULL, on_oauth_ready, &ctx);

  getchar();
  lb_quote_context_release(ctx);
  return 0;
}
