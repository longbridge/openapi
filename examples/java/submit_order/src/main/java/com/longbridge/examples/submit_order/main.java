import com.longbridge.*;
import com.longbridge.trade.*;
import java.math.BigDecimal;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv()) {
            try (TradeContext ctx = TradeContext.create(config).get()) {
                SubmitOrderOptions opts = new SubmitOrderOptions("700.HK",
                        OrderType.LO,
                        OrderSide.Buy,
                        200,
                        TimeInForceType.Day).setSubmittedPrice(new BigDecimal(50));
                SubmitOrderResponse resp = ctx.submitOrder(opts).join();
                System.out.println(resp);
            }
        }
    }
}
