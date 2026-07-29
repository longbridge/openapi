import com.longbridge.*;
import com.longbridge.trade.*;
import java.math.BigDecimal;

class Main {
    public static void main(String[] args) throws Exception {
        String clientId = "your-client-id";
        OAuth oauth = new OAuthBuilder(clientId)
                .build(url -> System.out.println("Open to authorize: " + url))
                .get();
        try (oauth;
             Config config = Config.fromOAuth(oauth);
             TradeContext ctx = TradeContext.create(config)) {
            SubmitAttachedParams attached = new SubmitAttachedParams(AttachedOrderType.BRACKET)
                    .setProfitTakerPrice(new BigDecimal("220"))
                    .setStopLossPrice(new BigDecimal("180"));

            SubmitOrderOptions opts = new SubmitOrderOptions(
                    "AAPL.US",
                    OrderType.LO,
                    OrderSide.Buy,
                    new BigDecimal("1"),
                    TimeInForceType.Day)
                    .setSubmittedPrice(new BigDecimal("200"))
                    .setAttachedParams(attached);

            SubmitOrderResponse resp = ctx.submitOrder(opts).get();
            System.out.println(resp);
        }
    }
}
