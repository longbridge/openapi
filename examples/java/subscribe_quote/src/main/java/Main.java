import com.longport.*;
import com.longport.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuth("your-client-id");
             OAuthToken token = oauth.authorize(url -> System.out.println(url)).get();
             Config config = Config.fromOAuth(token);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnQuote((symbol, event) -> {
                System.out.printf("%s\t%s\n", symbol, event);
            });
            ctx.subscribe(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" }, SubFlags.Quote).get();
            Thread.sleep(30000);
        }
    }
}
