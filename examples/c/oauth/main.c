#include <longport.h>
#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <curses.h>
#endif

static const char* CLIENT_ID = "your-client-id";

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
  const char* access_token = lb_oauth_token_get_access_token(token);
  uint64_t expires_at = lb_oauth_token_get_expires_at(token);

  printf("Access token: %s\n", access_token);
  printf("Expires at: %llu\n", (unsigned long long)expires_at);

  lb_config_t* config = lb_config_from_oauth(CLIENT_ID, access_token);
  printf("Config created successfully\n");
  lb_config_free(config);
}

int
main(int argc, char const* argv[])
{
#ifdef WIN32
  SetConsoleOutputCP(CP_UTF8);
#endif

  lb_error_t* err = NULL;
  lb_oauth_t* oauth = lb_oauth_new(CLIENT_ID);

  lb_oauth_authorize(oauth, on_open_url, NULL, on_oauth_authorize, NULL);
  getchar();

  lb_oauth_free(oauth);
  return 0;
}
