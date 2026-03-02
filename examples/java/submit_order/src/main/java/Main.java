import com.longport.*;
import com.longport.trade.*;
import java.math.BigDecimal;

public class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuth("your-client-id");
             OAuthToken token = oauth.authorize(url -> System.out.println(url)).get();
             Config config = Config.fromOauth("your-client-id", token.getAccessToken());
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
