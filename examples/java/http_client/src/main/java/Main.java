import com.longport.HttpClient;
import java.util.HashMap;

class Main {
    public static void main(String[] args) throws Exception {
        try (HttpClient httpCli = HttpClient.fromEnv()) {
            Object resp = httpCli.request(HashMap.class, "get", "/v1/trade/execution/today", null).get();
            System.out.println(resp);
        }
    }
}
