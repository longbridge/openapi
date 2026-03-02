import com.longport.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (OAuth oauth = new OAuth("your-client-id");
             OAuthToken token = oauth.authorize(url -> {
                 System.out.println("Open this URL to authorize: " + url);
             }).get()) {
            System.out.println("Access token: " + token.getAccessToken());
            System.out.println("Expires at: " + token.getExpiresAt());

            try (Config config = Config.fromOauth("your-client-id", token.getAccessToken())) {
                System.out.println("Config created successfully");
            }
        }
    }
}
