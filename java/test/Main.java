import com.longbridge.*;
import com.longbridge.trade.*;
import com.longbridge.quote.*;
import java.math.BigDecimal;
import java.util.Arrays;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
            WatchListGroup[] resp = ctx.getWatchList().get();
            System.out.println(Arrays.toString(resp));
        }
    }
}
