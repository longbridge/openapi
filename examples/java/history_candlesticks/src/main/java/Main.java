import java.time.LocalDateTime;
import java.time.LocalDate;
import java.util.Arrays;

import com.longport.*;
import com.longport.quote.*;

class Main {
    static OAuthToken getToken() throws Exception {
        String clientId = "your-client-id";
        try {
            OAuthToken token = OAuthToken.load().get();
            if (token.isExpired()) throw new Exception("token expired");
            if (token.expiresSoon()) {
                try (OAuth oauth = new OAuth(clientId)) {
                    OAuthToken newToken = oauth.refresh(token).get();
                    newToken.save().get();
                    return newToken;
                } catch (Exception e) { /* fall through */ }
            } else {
                return token;
            }
        } catch (Exception e) { /* fall through */ }
        try (OAuth oauth = new OAuth(clientId)) {
            OAuthToken token = oauth.authorize(url -> System.out.println(url)).get();
            token.save().get();
            return token;
        }
    }

    public static void main(String[] args) throws Exception {
        OAuthToken token = getToken();
        try (token;
             Config config = Config.fromOAuth(token);
             QuoteContext ctx = QuoteContext.create(config).get()) {
            System.out.println("get candlesticks by offset");
            System.out.println("====================");

            Candlestick[] candlesticks = ctx
                            .getHistoryCandlesticksByOffset("700.HK", Period.Day, AdjustType.NoAdjust,
                                            false,
                                            LocalDateTime.of(2023, 8, 18, 0, 0, 0, 0), 10,
                                            TradeSessions.Intraday)
                            .get();
            System.out.println(Arrays.toString(candlesticks));

            System.out.println("get candlesticks by date");
            System.out.println("====================");

            Candlestick[] candlesticks2 = ctx
                            .getHistoryCandlesticksByDate("700.HK", Period.Day, AdjustType.NoAdjust,
                                            LocalDate.of(2022, 5, 5), LocalDate.of(2022, 6, 23),
                                            TradeSessions.Intraday)
                            .get();
            System.out.println(Arrays.toString(candlesticks2));
        }
    }
}
