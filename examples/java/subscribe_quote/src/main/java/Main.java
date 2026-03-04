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
            ctx.setOnQuote((symbol, event) -> {
                System.out.printf("%s\t%s\n", symbol, event);
            });
            ctx.subscribe(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }, SubFlags.Quote).get();
            Thread.sleep(30000);
        }
    }
}
