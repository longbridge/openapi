import com.longbridge.*;
import com.longbridge.trade.*;
import java.math.BigDecimal;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
            GetHistoryOrdersOptions opts = new GetHistoryOrdersOptions();
            Order[] resp = ctx.getHistoryOrders(opts).get();
            System.out.println(resp[0].toString());
        }
    }
}
