import com.longbridge.*;
import com.longbridge.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        String clientId = "your-client-id";
        OAuth oauth = new OAuthBuilder(clientId)
                .build(url -> System.out.println("Open to authorize: " + url))
                .get();
        try (oauth;
             Config config = Config.fromOAuth(oauth);
             TradeContext ctx = TradeContext.create(config).get()) {
            for (AccountBalance obj : ctx.getAccountBalance().get()) {
                System.out.println(obj);
            }
        }
    }
}
