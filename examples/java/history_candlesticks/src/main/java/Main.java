import java.time.LocalDateTime;
import java.time.LocalDate;
import java.util.Arrays;

import com.longport.*;
import com.longport.quote.*;

class Main {
        public static void main(String[] args) throws Exception {
                try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
                        System.out.println("get candlesticks by offset");
                        System.out.println("====================");

                        Candlestick[] candlesticks = ctx
                                        .getHistoryCandlesticksByOffset("700.HK", Period.Day, AdjustType.NoAdjust,
                                                        false,
                                                        LocalDateTime.of(2023, 8, 18, 0, 0, 0, 0), 10,
                                                        TradeSessions.Normal)
                                        .get();
                        System.out.println(Arrays.toString(candlesticks));

                        System.out.println("get candlesticks by date");
                        System.out.println("====================");

                        Candlestick[] candlesticks2 = ctx
                                        .getHistoryCandlesticksByDate("700.HK", Period.Day, AdjustType.NoAdjust,
                                                        LocalDate.of(2022, 5, 5), LocalDate.of(2022, 6, 23),
                                                        TradeSessions.Normal)
                                        .get();
                        System.out.println(Arrays.toString(candlesticks2));
                }
        }
}
