import com.longport.*;
import java.util.HashMap;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuth("your-client-id");
             OAuthToken token = oauth.authorize(url -> System.out.println(url)).get()) {
            HttpClient httpCli = new HttpClient(
                "https://openapi.longportapp.com", "", "", token.getAccessToken());
            try {
                Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
                System.out.println(resp);
            } finally {
                httpCli.close();
            }
        }
    }
}
