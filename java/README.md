# LongPort OpenAPI SDK for Java

`longport` provides an easy-to-use interface for invoking [`LongPort OpenAPI`](https://open.longportapp.com/en/).

## Documentation

- SDK docs: https://longportapp.github.io/openapi/java/index.html
- LongPort OpenAPI: https://open.longportapp.com/en/

## Examples

Runnable examples live in `examples/java/`:

- `examples/java/account_asset/src/main/java/main.java`
- `examples/java/history_candlesticks/src/main/java/Main.java`
- `examples/java/subscribe_quote/src/main/java/Main.java`
- `examples/java/submit_order/src/main/java/Main.java`
- `examples/java/today_orders/src/main/java/main.java`

## Quickstart

_Install LongPort OpenAPI SDK_

Add `io.github.longportapp:openapi-sdk` to `pom.xml`

```xml
<dependencies>
    <dependency>
        <groupId>io.github.longportapp</groupId>
        <artifactId>openapi-sdk</artifactId>
        <version>LATEST</version>
    </dependency>
</dependencies>
```

### Authentication

LongPort OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Authorize, Refresh, and Get Token**

```java
import com.longport.*;

class Main {
    static OAuthToken getToken() throws Exception {
        OAuthToken token = null;
        try {
            token = OAuthToken.load().get();
        } catch (Exception e) {
            // no saved token
        }

        if (token == null || token.isExpired()) {
            // No saved token or token has expired — re-authorize
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
            }
            token.save().get();
        } else if (token.expiresSoon()) {
            // Token will expire soon — refresh it
            try (OAuth oauth = new OAuth("your-client-id")) {
                try {
                    token = oauth.refresh(token).get();
                    token.save().get();
                } catch (Exception e) {
                    // Refresh failed — fall back to re-authorization
                    token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
                    token.save().get();
                }
            }
        }
        return token;
    }

    public static void main(String[] args) throws Exception {
        OAuthToken token = getToken();
        Config config = Config.fromOAuth(token);
        // Use config to create contexts...
    }
}
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGPORT_APP_KEY="App Key get from user center"
export LONGPORT_APP_SECRET="App Secret get from user center"
export LONGPORT_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGPORT_APP_KEY "App Key get from user center"
setx LONGPORT_APP_SECRET "App Secret get from user center"
setx LONGPORT_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```java
import com.longport.*;
import com.longport.quote.*;
import java.math.BigDecimal;

class Main {
    static OAuthToken getToken() throws Exception {
        OAuthToken token = null;
        try { token = OAuthToken.load().get(); } catch (Exception e) {}
        if (token == null || token.isExpired()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
            }
            token.save().get();
        } else if (token.expiresSoon()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                try {
                    token = oauth.refresh(token).get(); token.save().get();
                } catch (Exception e) {
                    token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
                    token.save().get();
                }
            }
        }
        return token;
    }

    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromOAuth(getToken());
             QuoteContext ctx = QuoteContext.create(config).get()) {
            SecurityQuote[] resp = ctx.getQuote(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }).get();
            for (SecurityQuote obj : resp) {
                System.out.println(obj);
            }
        }
    }
}
```

## Quote API _(Subscribe quotes)_

```java
import com.longport.*;
import com.longport.quote.*;

class Main {
    static OAuthToken getToken() throws Exception {
        OAuthToken token = null;
        try { token = OAuthToken.load().get(); } catch (Exception e) {}
        if (token == null || token.isExpired()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
            }
            token.save().get();
        } else if (token.expiresSoon()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                try {
                    token = oauth.refresh(token).get(); token.save().get();
                } catch (Exception e) {
                    token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
                    token.save().get();
                }
            }
        }
        return token;
    }

    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromOAuth(getToken());
             QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnQuote((symbol, quote) -> {
                System.out.printf("%s\t%s\n", symbol, quote);
            });
            ctx.subscribe(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }, SubFlags.Quote).get();
            Thread.sleep(30000);
        }
    }
}
```

## Trade API _(Submit order)_

```java
import com.longport.*;
import com.longport.trade.*;
import java.math.BigDecimal;

public class Main {
    static OAuthToken getToken() throws Exception {
        OAuthToken token = null;
        try { token = OAuthToken.load().get(); } catch (Exception e) {}
        if (token == null || token.isExpired()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
            }
            token.save().get();
        } else if (token.expiresSoon()) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                try {
                    token = oauth.refresh(token).get(); token.save().get();
                } catch (Exception e) {
                    token = oauth.authorize(url -> System.out.println("Open this URL to authorize: " + url)).get();
                    token.save().get();
                }
            }
        }
        return token;
    }

    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromOAuth(getToken());
             TradeContext ctx = TradeContext.create(config).get()) {
            SubmitOrderOptions opts = new SubmitOrderOptions("700.HK",
                    OrderType.LO,
                    OrderSide.Buy,
                    200,
                    TimeInForceType.Day).setSubmittedPrice(new BigDecimal(50));
            SubmitOrderResponse resp = ctx.submitOrder(opts).get();
            System.out.println(resp);
        }
    }
}
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If you don't see push events, ensure the program keeps running (e.g. `Thread.sleep(...)`).
- For debugging, set `LONGPORT_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
