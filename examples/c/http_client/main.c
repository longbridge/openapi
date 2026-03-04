#include <longbridge.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

void
on_response(const struct lb_async_result_t* res)
{
  if (res->error) {
    printf("failed: %s\n", lb_error_message(res->error));
    return;
  }

  const lb_http_result_t* resp = (const lb_http_result_t*)res->data;
  printf("%s\n", lb_http_result_response_body(resp));
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
  lb_http_client_t* http_client = lb_http_client_from_oauth(oauth, NULL);
  lb_oauth_free((lb_oauth_t*)oauth);
  lb_http_client_request(http_client,
                         "get",
                         "/v1/trade/execution/today",
                         NULL,
                         NULL,
                         on_response,
                         NULL);
  getchar();
  lb_http_client_free(http_client);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_oauth_new(CLIENT_ID, 0, on_open_url, NULL, on_oauth_ready, NULL);

  return 0;
}
