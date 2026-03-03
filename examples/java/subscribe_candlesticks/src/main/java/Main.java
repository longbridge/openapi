import com.longport.*;
import com.longport.quote.*;

class Main {
    static OAuthToken getToken() throws Exception {
        String clientId = "your-client-id";
        try {
            OAuthToken token = OAuthToken.load().get();
            if (token.isExpired()) throw new Exception("token expired");
            if (token.expiresSoon()) {
                try (OAuth oauth = new OAuth(clientId)) {
                    OAuthToken newToken = oauth.refresh(token).get();
                    newToken.save().get();
                    return newToken;
                } catch (Exception e) { /* fall through */ }
            } else {
                return token;
            }
        } catch (Exception e) { /* fall through */ }
        try (OAuth oauth = new OAuth(clientId)) {
            OAuthToken token = oauth.authorize(url -> System.out.println(url)).get();
            token.save().get();
            return token;
        }
    }

    public static void main(String[] args) throws Exception {
        OAuthToken token = getToken();
        try (token;
             Config config = Config.fromOAuth(token);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnCandlestick((symbol, event) -> {
                System.out.printf("%s\t%s\n", symbol, event);
            });
            ctx.subscribeCandlesticks("AAPL.US", Period.Min_1, TradeSessions.Intraday).get();
            Thread.sleep(30000);
        }
    }
}
