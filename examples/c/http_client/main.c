#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

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

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;
  lb_http_client_t* http_client = lb_http_client_from_env(&err);
  if (err) {
    printf("failed to create http client from environment: %s\n",
           lb_error_message(err));
    lb_error_free(err);
    return -1;
  }

  lb_http_client_request(http_client,
                         "get",
                         "/v1/trade/execution/today",
                         NULL,
                         NULL,
                         on_response,
                         NULL);
  getchar();
  lb_http_client_free(http_client);
  return 0;
}