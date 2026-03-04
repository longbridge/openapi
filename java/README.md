# Longbridge OpenAPI SDK for Java

`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).

## Documentation

- SDK docs: https://longbridge.github.io/openapi/java/index.html
- Longbridge OpenAPI: https://open.longbridge.com/en/

## Examples

Runnable examples live in `examples/java/`:

- `examples/java/account_asset/src/main/java/main.java`
- `examples/java/history_candlesticks/src/main/java/Main.java`
- `examples/java/subscribe_quote/src/main/java/Main.java`
- `examples/java/submit_order/src/main/java/Main.java`
- `examples/java/today_orders/src/main/java/main.java`

## Quickstart

_Install Longbridge OpenAPI SDK_

Add `io.github.longbridge:openapi-sdk` to `pom.xml`

```xml
<dependencies>
    <dependency>
        <groupId>io.github.longbridge</groupId>
        <artifactId>openapi-sdk</artifactId>
        <version>LATEST</version>
    </dependency>
</dependencies>
```

### Authentication

Longbridge OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 is the modern authentication method that uses Bearer tokens without requiring HMAC signatures.

**Step 1: Register OAuth Client**

First, register an OAuth client to get your `client_id`:

_bash / macOS / Linux_

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

_PowerShell (Windows)_

```powershell
Invoke-RestMethod -Method Post -Uri https://openapi.longbridgeapp.com/oauth2/register `
  -ContentType "application/json" `
  -Body '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:
```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "client_name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

Save the `client_id` for use in your application.

**Step 2: Build OAuth client and create a Config**

`OAuthBuilder.build` loads a cached token from
`~/.longbridge-openapi/tokens/<client_id>` (`%USERPROFILE%\.longbridge-openapi\tokens\<client_id>` on Windows)
if one exists and is still valid, or starts the browser authorization flow
automatically.  The token is persisted to the same path after a successful
authorization or refresh.  The resulting `OAuth` handle is passed directly to
`Config.fromOAuth`.

```java
import com.longbridge.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuthBuilder("your-client-id")
                .build(url -> System.out.println("Open this URL to authorize: " + url))
                .get();
             Config config = Config.fromOAuth(oauth)) {
            // Use config to create contexts...
        }
    }
}
```

#### 2. Legacy API Key (Environment Variables)

_Setting environment variables(MacOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```java
import com.longbridge.*;
import com.longbridge.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuthBuilder("your-client-id")
                .build(url -> System.out.println("Open this URL to authorize: " + url))
                .get();
             Config config = Config.fromOAuth(oauth);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            SecurityQuote[] resp = ctx.getQuote(
                    new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }).get();
            for (SecurityQuote obj : resp) {
                System.out.println(obj);
            }
        }
    }
}
```

## Quote API _(Subscribe quotes)_

```java
import com.longbridge.*;
import com.longbridge.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuthBuilder("your-client-id")
                .build(url -> System.out.println("Open this URL to authorize: " + url))
                .get();
             Config config = Config.fromOAuth(oauth);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnQuote((symbol, quote) -> System.out.printf("%s\t%s\n", symbol, quote));
            ctx.subscribe(
                    new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" },
                    SubFlags.Quote).get();
            Thread.sleep(30000);
        }
    }
}
```

## Trade API _(Submit order)_

```java
import com.longbridge.*;
import com.longbridge.trade.*;
import java.math.BigDecimal;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuthBuilder("your-client-id")
                .build(url -> System.out.println("Open this URL to authorize: " + url))
                .get();
             Config config = Config.fromOAuth(oauth);
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
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
