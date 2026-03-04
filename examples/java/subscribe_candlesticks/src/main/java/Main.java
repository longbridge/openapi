import com.longbridge.*;
import com.longbridge.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        String clientId = "your-client-id";
        OAuth oauth = new OAuthBuilder(clientId)
                .build(url -> System.out.println("Open to authorize: " + url))
                .get();
        try (oauth;
             Config config = Config.fromOAuth(oauth);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnCandlestick((symbol, event) -> {
                System.out.printf("%s\t%s\n", symbol, event);
            });
            ctx.subscribeCandlesticks("AAPL.US", Period.Min_1, TradeSessions.Intraday).get();
            Thread.sleep(30000);
        }
    }
}
