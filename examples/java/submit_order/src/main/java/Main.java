import com.longport.*;
import com.longport.trade.*;
import java.math.BigDecimal;

public class Main {
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
            SubmitOrderOptions opts = new SubmitOrderOptions("700.HK",
                    OrderType.LO,
                    OrderSide.Buy,
                    new BigDecimal(200),
                    TimeInForceType.Day).setSubmittedPrice(new BigDecimal(50));
            SubmitOrderResponse resp = ctx.submitOrder(opts).get();
            System.out.println(resp);
        }
    }
}
