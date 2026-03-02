import com.longport.*;
import java.util.HashMap;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuth("your-client-id");
             OAuthToken token = oauth.authorize(url -> System.out.println(url)).get()) {
            try (HttpClient httpCli = HttpClient.fromOauth(oauth.getClientId(), token.getAccessToken())) {
                Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
                System.out.println(resp);
            }
        }
    }
}
