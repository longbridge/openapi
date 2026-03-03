import com.longport.*;
import java.util.HashMap;

class Main {
    public static void main(String[] args) throws Exception {
        OAuthToken token;
        try {
            token = OAuthToken.load().get();
        } catch (Exception e) {
            try (OAuth oauth = new OAuth("your-client-id")) {
                token = oauth.authorize(url -> System.out.println(url)).get();
            }
            token.save().get();
        }
        try (token) {
            try (HttpClient httpCli = HttpClient.fromOAuth(token)) {
                Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
                System.out.println(resp);
            }
        }
    }
}
