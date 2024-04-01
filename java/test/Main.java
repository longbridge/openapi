import com.longport.*;
import com.longport.quote.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (Config config = Config.fromEnv();
                QuoteContext ctx = QuoteContext.create(config).get()) {
            QueryWarrantOptions opts = new QueryWarrantOptions("700.HK",
                    WarrantSortBy.LastDone,
                    SortOrderType.Ascending)
                    .setExpiryDate(new FilterWarrantExpiryDate[] { FilterWarrantExpiryDate.LT_3 })
                    .setStatus(new WarrantStatus[] { WarrantStatus.Normal });
            WarrantInfo[] resp = ctx.queryWarrantList(opts).get();
            for (WarrantInfo obj : resp) {
                System.out.println(obj);
            }
        }
    }
}
