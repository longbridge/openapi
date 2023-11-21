import com.longport.*;
import com.longport.trade.*;
import com.longport.quote.*;
import java.math.BigDecimal;
import java.util.Arrays;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
            SecurityStaticInfo[] resp = ctx.getStaticInfo(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" })
                    .get();
            System.out.println(Arrays.toString(resp));
        }
    }
}
