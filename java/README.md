# Longbridge OpenAPI SDK for Java

`longbridge` provides an easy-to-use interface for invokes [`Longbridge OpenAPI`](https://open.longbridgeapp.com/en/).

## Quickstart

_Install Longbridge OpenAPI SDK_

Add `io.github.longbridgeapp:openapi-sdk` to `pom.xml`

```xml
<dependencies>
    <dependency>
        <groupId>io.github.longbridgeapp</groupId>
        <artifactId>openapi-sdk</artifactId>
        <version>0.2.19</version>
    </dependency>
</dependencies>
```

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
import com.longbridge.*;
import com.longbridge.quote.*;

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
import com.longbridge.*;
import com.longbridge.trade.*;
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

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
