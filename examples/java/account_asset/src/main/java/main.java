import com.longport.*;
import com.longport.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        OAuthToken token;
        try {
            token = OAuthToken.load().get();
        } catch (Exception e) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println(url)).get();
            }
            token.save().get();
        }
        try (token;
             Config config = Config.fromOAuth(token);
             TradeContext ctx = TradeContext.create(config).get()) {
            for (AccountBalance obj : ctx.getAccountBalance().get()) {
                System.out.println(obj);
            }
        }
    }
}
