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
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
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
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnQuote((symbol, quote) -> {
                System.out.printf("%s\t%s\n", symbol, quote);
            });
            ctx.subscribe(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }, SubFlags.Quote, true).get();
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
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
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
