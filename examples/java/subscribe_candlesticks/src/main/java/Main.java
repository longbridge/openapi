import com.longport.*;
import com.longport.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
            ctx.setOnCandlestick((symbol, event) -> {
                System.out.printf("%s\t%s\n", symbol, event);
            });
            ctx.subscribeCandlesticks("AAPL.US", Period.Min_1).get();
            Thread.sleep(30000);
        }
    }
}