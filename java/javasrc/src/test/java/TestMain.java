import com.longbridge.*;
import com.longbridge.quote.*;
import org.junit.Test;

public class TestMain {
    @Test
    public void main() {
        try {
            Config config = Config.fromEnv();
            QuoteContext ctx = QuoteContext.create(config).get();
            ctx.setOnQuote((symbol, quote) -> {
                System.out.printf("%s %s\n", symbol, quote.getLastDone());
            });
            ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Quote, true).get();
        } catch (Exception e) {
            System.out.println(e);
        }
    }
}
