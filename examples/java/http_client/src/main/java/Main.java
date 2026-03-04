import com.longbridge.*;
import java.util.HashMap;

class Main {
    public static void main(String[] args) throws Exception {
        String clientId = "your-client-id";
        OAuth oauth = new OAuthBuilder(clientId)
                .build(url -> System.out.println("Open to authorize: " + url))
                .get();
        try (oauth;
             HttpClient httpCli = HttpClient.fromOAuth(oauth)) {
            Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
            System.out.println(resp);
        }
    }
}
