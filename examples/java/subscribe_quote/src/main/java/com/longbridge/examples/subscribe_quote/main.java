import com.longbridge.*;
import com.longbridge.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv()) {
            try (QuoteContext ctx = QuoteContext.create(config).get()) {
                ctx.setOnQuote((symbol, quote) -> {
                    System.out.println("%s\t%s\n", symbol, quote);
                });
                ctx.subscribe(new String[] { "700.HK1", "AAPL.US", "TSLA.US", "NFLX.US" }, SubFlags.Quote, true).get();
                Thread.sleep(30000);
            }
        }
    }
}