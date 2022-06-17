import com.longbridge.*;
import com.longbridge.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
            StockPositionsResponse resp = ctx.getStockPositions(null).get();
            System.out.println(resp);
        }
    }
}
