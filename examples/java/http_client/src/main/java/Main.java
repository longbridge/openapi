import com.longport.*;
import java.util.HashMap;

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
        try (token) {
            try (HttpClient httpCli = HttpClient.fromOAuth(token)) {
                Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
                System.out.println(resp);
            }
        }
    }
}
