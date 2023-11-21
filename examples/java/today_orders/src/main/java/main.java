import com.longport.*;
import com.longport.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
            Order[] orders = ctx.getTodayOrders(null).get();
            for (Order order : orders) {
                System.out.println(order);
            }
        }
    }
}