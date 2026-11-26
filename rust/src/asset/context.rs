use std::sync::Arc;

use longbridge_httpcli::{DcRegion, HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    asset::{
        BankCard, BankCardsResponse, CouponOrder, CouponOrderType, CouponOrdersResponse,
        CouponsResponse, DepositsResponse, EddaApplication, EddaApplicationsResponse,
        GetCouponsOptions, GetDepositsOptions, GetRewardRecordsOptions, GetStatementListOptions,
        GetStatementListResponse, GetStatementOptions, GetStatementResponse,
        GetStockTransfersInOptions, GetStockTransfersOutOptions, GetWithdrawalsOptions,
        RewardRecordsResponse, StockTransferInResponse, StockTransferOut, StockTransferOutResponse,
        WithdrawalsResponse, core,
        requests::{GetCouponOrdersQuery, GetEddaApplicationsQuery},
        token,
    },
};

struct InnerAssetContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerAssetContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("asset context dropped");
        });
    }
}

/// Asset context
#[derive(Clone)]
pub struct AssetContext(Arc<InnerAssetContext>);

impl AssetContext {
    /// Create a `AssetContext`
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("asset");

        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating asset context");
        });

        let ctx = Self(Arc::new(InnerAssetContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));

        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("asset context created");
        });

        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    async fn get<R, Q>(&self, path: &'static str, query: Q) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        Q: Serialize + Send + Sync,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(query)
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// `GET` an account (ATM / reward-center) route. These routes are served
    /// by the AP data center only, so a US-account session fails fast with a
    /// region error instead of reaching a server that rejects it. The
    /// account channel read from the access token is sent as
    /// `account_channel`, as the app does (left out when the token does not
    /// decode).
    async fn get_account<R, Q>(&self, path: &'static str, query: Q) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        Q: Serialize + Send + Sync,
    {
        self.get_account_route(path, query, true).await
    }

    /// [`Self::get_account`] with `account_channel` sent only when
    /// `send_channel` is set.
    async fn get_account_route<R, Q>(
        &self,
        path: &'static str,
        query: Q,
        send_channel: bool,
    ) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        Q: Serialize + Send + Sync,
    {
        #[derive(Serialize)]
        struct AccountQuery<Q> {
            #[serde(flatten)]
            query: Q,
            #[serde(skip_serializing_if = "Option::is_none")]
            account_channel: Option<String>,
        }

        let account_channel = if send_channel {
            self.0
                .http_cli
                .access_token()
                .await
                .ok()
                .and_then(|token| token::account_channel(&token))
        } else {
            None
        };
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .dc_restrict(DcRegion::Ap)
            .query_params(AccountQuery {
                query,
                account_channel,
            })
            .response::<Json<R>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get statement data list
    ///
    /// Path: GET /v1/statement/list
    pub async fn statements(
        &self,
        options: GetStatementListOptions,
    ) -> Result<GetStatementListResponse> {
        self.get(core::GET_STATEMENT_DATA_LIST_PATH, options).await
    }

    /// Get statement data download url
    ///
    /// Path: GET /v1/statement/download
    pub async fn statement_download_url(
        &self,
        options: GetStatementOptions,
    ) -> Result<GetStatementResponse> {
        self.get(core::GET_STATEMENT_DATA_DOWNLOAD_URL_PATH, options)
            .await
    }

    /// Get the linked bank cards, with their verification, binding and
    /// authorization (eDDA, bank securities account, FPS) status
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v2/account/bank-cards`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, asset::AssetContext, oauth::OAuthBuilder};
    ///
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let oauth = OAuthBuilder::new("your-client-id")
    ///     .build(|url| println!("Visit: {url}"))
    ///     .await?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let ctx = AssetContext::new(config);
    ///
    /// for card in ctx.bank_cards().await? {
    ///     println!("{} {:?}", card.bank, card.status);
    /// }
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn bank_cards(&self) -> Result<Vec<BankCard>> {
        #[derive(Serialize)]
        struct Query {
            from_app_page: &'static str,
        }
        Ok(self
            .get_account::<BankCardsResponse, _>(
                core::BANK_CARDS_PATH,
                Query {
                    from_app_page: "cards",
                },
            )
            .await?
            .list)
    }

    /// Get the eDDA authorization applications of a bank card in one
    /// currency
    ///
    /// `card_id` is [`BankCard::id`]; `currency` is e.g. `HKD` or `USD`.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/edda-status`
    pub async fn edda_applications(
        &self,
        card_id: impl Into<String>,
        currency: impl Into<String>,
    ) -> Result<Vec<EddaApplication>> {
        let query = GetEddaApplicationsQuery {
            card_id: card_id.into().trim().to_string(),
            currency: currency.into().trim().to_uppercase(),
        };
        Ok(self
            .get_account::<EddaApplicationsResponse, _>(core::EDDA_STATUS_PATH, query)
            .await?
            .0)
    }

    /// Get stock transfer-in applications (another broker → this account)
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/stock-transfers/in`
    pub async fn stock_transfers_in(
        &self,
        options: impl Into<Option<GetStockTransfersInOptions>>,
    ) -> Result<StockTransferInResponse> {
        self.get_account(
            core::STOCK_TRANSFERS_IN_PATH,
            options.into().unwrap_or_default(),
        )
        .await
    }

    /// Get stock transfer-out applications (this account → another broker)
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/stock-transfers/out`
    pub async fn stock_transfers_out(
        &self,
        options: impl Into<Option<GetStockTransfersOutOptions>>,
    ) -> Result<Vec<StockTransferOut>> {
        Ok(self
            .get_account::<StockTransferOutResponse, _>(
                core::STOCK_TRANSFERS_OUT_PATH,
                options.into().unwrap_or_default(),
            )
            .await?
            .records)
    }

    /// Get reward records from the reward center (cash, gifts, coupons, task
    /// rewards)
    ///
    /// The route has no date filter: to find rewards in a time range, page
    /// through the records and filter on their times.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/rewards`
    pub async fn reward_records(
        &self,
        options: impl Into<Option<GetRewardRecordsOptions>>,
    ) -> Result<RewardRecordsResponse> {
        self.get_account(core::REWARDS_PATH, options.into().unwrap_or_default())
            .await
    }

    /// Get coupons (reward cards)
    ///
    /// By default lists the coupons that have not expired; see
    /// [`GetCouponsOptions::expired`]. The pinned `top_list` / `bottom_list`
    /// are repeated on every page.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/cards`
    pub async fn coupons(
        &self,
        options: impl Into<Option<GetCouponsOptions>>,
    ) -> Result<CouponsResponse> {
        self.get_account(core::COUPONS_PATH, options.into().unwrap_or_default())
            .await
    }

    /// Get the orders a coupon was applied to
    ///
    /// `card_id` is [`crate::asset::Coupon::card_id`] (32 hex characters, not a
    /// bank card ID). The gateway rate-limits this route tightly per member, so
    /// avoid calling it concurrently.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/card-orders`
    pub async fn coupon_orders(
        &self,
        card_id: impl Into<String>,
        order_type: CouponOrderType,
    ) -> Result<Vec<CouponOrder>> {
        let query = GetCouponOrdersQuery {
            card_id: card_id.into().trim().to_string(),
            order_type: order_type.as_str(),
        };
        Ok(self
            // Like the app and the MCP server, the coupon-orders route is
            // called without an account channel.
            .get_account_route::<CouponOrdersResponse, _>(core::COUPON_ORDERS_PATH, query, false)
            .await?
            .orders)
    }

    /// Get deposit records
    ///
    /// The server's `total` follows the page size rather than the record
    /// count, so it is not returned; [`DepositsResponse::has_more`] reports
    /// a full page instead. The page size is at most 20 and defaults to 20
    /// (see [`GetDepositsOptions::size`]); it is always sent.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/deposits`
    pub async fn deposits(
        &self,
        options: impl Into<Option<GetDepositsOptions>>,
    ) -> Result<DepositsResponse> {
        let options = options.into().unwrap_or_default();
        let page_size = options.page_size();
        let mut resp: DepositsResponse = self
            .get_account(core::DEPOSITS_PATH, options.size(page_size))
            .await?;
        resp.has_more = usize::try_from(page_size).is_ok_and(|size| resp.items.len() >= size);
        Ok(resp)
    }

    /// Get withdrawal records
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/withdrawals`
    pub async fn withdrawals(
        &self,
        options: impl Into<Option<GetWithdrawalsOptions>>,
    ) -> Result<WithdrawalsResponse> {
        self.get_account(core::WITHDRAWALS_PATH, options.into().unwrap_or_default())
            .await
    }
}

#[cfg(test)]
mod tests {
    use longbridge_httpcli::HttpClientError;
    use time::macros::datetime;

    use super::*;
    use crate::{
        Error,
        asset::{DepositState, RewardType, StockTransferInState, StockTransferOutState},
    };

    fn query(value: &impl Serialize) -> serde_json::Value {
        serde_json::to_value(value).expect("options serialize")
    }

    fn json(text: &str) -> serde_json::Value {
        serde_json::from_str(text).expect("valid JSON literal")
    }

    #[test]
    fn account_options_serialize_upstream_names() {
        let q = query(
            &GetStockTransfersInOptions::new()
                .market(" hk ")
                .state(StockTransferInState::Completed)
                .start(datetime!(2026-01-01 0:00 +8))
                .page(2)
                .size(50),
        );
        assert_eq!(
            q,
            json(r#"{"market":"HK","state":3,"start":1767196800,"page":2,"size":50}"#),
            "transfer options"
        );

        let q = query(
            &GetDepositsOptions::new()
                .states([
                    DepositState::Processing,
                    DepositState::Unknown,
                    DepositState::Failed,
                ])
                .currencies(["usd", "HKD"])
                .start(datetime!(2026-01-01 0:00 +8))
                .end(datetime!(2026-01-01 23:59:59 +8)),
        );
        assert_eq!(
            q,
            json(
                r#"{"states":"0,2","currencies":"USD,HKD","from_date":1767196800,"to_date":1767283199}"#
            ),
            "deposit options"
        );
        assert_eq!(
            query(&GetDepositsOptions::new().size(50)),
            json(r#"{"size":20}"#),
            "deposit page size clamped to 20"
        );
        assert_eq!(
            query(&GetDepositsOptions::new().size(15)),
            json(r#"{"size":15}"#),
            "smaller deposit page size kept"
        );
        assert_eq!(
            query(&GetDepositsOptions::new().size(0)),
            json(r#"{"size":20}"#),
            "zero page size is the default"
        );
        assert_eq!(
            query(&GetDepositsOptions::new().size(-3)),
            json(r#"{"size":20}"#),
            "negative page size is the default"
        );
        assert_eq!(
            GetDepositsOptions::new().page_size(),
            20,
            "default page size"
        );

        let q = query(&GetWithdrawalsOptions::new().start(datetime!(2026-01-01 0:00 +8)));
        assert_eq!(q, json(r#"{"begin":1767196800}"#), "withdrawal options");

        let q = query(
            &GetRewardRecordsOptions::new()
                .page(1)
                .page_size(20)
                .reward_type(RewardType::Coupon),
        );
        assert_eq!(
            q,
            json(r#"{"current":1,"page_size":20,"reward_type":2}"#),
            "reward options"
        );

        let q = query(
            &GetStockTransfersOutOptions::new()
                .state(StockTransferOutState::Failed)
                .end(datetime!(2026-01-01 23:59:59 +8)),
        );
        assert_eq!(
            q,
            json(r#"{"state":6,"end":1767283199}"#),
            "transfer-out state sent as its server code"
        );
        assert_eq!(
            query(&GetStockTransfersInOptions::new().state(StockTransferInState::Failed)),
            json(r#"{"state":8}"#),
            "transfer-in failed is code 8"
        );
        assert_eq!(
            query(&GetStockTransfersInOptions::new().state(StockTransferInState::Unknown)),
            json("{}"),
            "unknown state is not sent"
        );
        assert_eq!(
            query(&GetRewardRecordsOptions::new().reward_type(RewardType::Unknown)),
            json("{}"),
            "unknown reward type is not sent"
        );

        assert_eq!(
            query(&GetCouponsOptions::new()),
            json(r#"{"card_status":-1}"#),
            "valid coupons by default"
        );
        assert_eq!(
            query(
                &GetCouponsOptions::new()
                    .expired(true)
                    .card_type("commission")
            ),
            json(r#"{"card_status":2,"card_type":"commission"}"#),
            "expired coupons"
        );
    }

    /// The account routes send the token's account channel, except the
    /// coupon-orders route (as in the app and the MCP server).
    #[tokio::test(flavor = "multi_thread")]
    async fn account_routes_send_the_account_channel() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind a local port");
        let addr = listener.local_addr().expect("local address");
        let server = tokio::spawn(async move {
            let mut lines = Vec::new();
            for _ in 0..2 {
                let (mut socket, _) = listener.accept().await.expect("accept");
                let mut buf = vec![0u8; 8192];
                let n = socket.read(&mut buf).await.expect("read request");
                let body = r#"{"code":0,"message":"","data":{}}"#;
                let reply = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                socket.write_all(reply.as_bytes()).await.expect("reply");
                let request = String::from_utf8_lossy(&buf[..n]).to_string();
                lines.push(request.lines().next().unwrap_or_default().to_string());
            }
            lines
        });
        // A synthetic unsigned JWT: {"alg":"none"} . {"ac":"lb_test"} . sig
        let token = "eyJhbGciOiJub25lIn0.eyJhYyI6ImxiX3Rlc3QifQ.sig";
        let config = Arc::new(
            Config::from_apikey("app_key", "app_secret", token).http_url(format!("http://{addr}")),
        );
        let ctx = AssetContext::new(config);
        let coupons = ctx.coupons(None).await.expect("coupons");
        assert!(coupons.list.is_empty(), "empty list");
        let orders = ctx
            .coupon_orders("c1", CouponOrderType::Locked)
            .await
            .expect("coupon orders");
        assert!(orders.is_empty(), "empty list");
        let lines = server.await.expect("server task");
        assert!(
            lines[0].starts_with("GET /v1/account/cards?"),
            "{}",
            lines[0]
        );
        assert!(
            lines[0].contains("card_status=-1"),
            "own query kept: {}",
            lines[0]
        );
        assert!(
            lines[0].contains("account_channel=lb_test"),
            "channel sent: {}",
            lines[0]
        );
        assert!(
            lines[1].starts_with("GET /v1/account/card-orders?"),
            "{}",
            lines[1]
        );
        assert!(
            lines[1].contains("card_id=c1"),
            "own query kept: {}",
            lines[1]
        );
        assert!(
            !lines[1].contains("account_channel"),
            "no channel on card-orders: {}",
            lines[1]
        );
    }

    /// `has_more` follows the page size actually sent.
    #[tokio::test(flavor = "multi_thread")]
    async fn deposits_report_a_full_page() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind a local port");
        let addr = listener.local_addr().expect("local address");
        let server = tokio::spawn(async move {
            let mut lines = Vec::new();
            for count in [20, 15] {
                let (mut socket, _) = listener.accept().await.expect("accept");
                let mut buf = vec![0u8; 8192];
                let n = socket.read(&mut buf).await.expect("read request");
                let items = vec![r#"{"id":"1"}"#; count].join(",");
                let body = format!(r#"{{"code":0,"message":"","data":{{"items":[{items}]}}}}"#);
                let reply = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                socket.write_all(reply.as_bytes()).await.expect("reply");
                let request = String::from_utf8_lossy(&buf[..n]).to_string();
                lines.push(request.lines().next().unwrap_or_default().to_string());
            }
            lines
        });
        let config = Arc::new(
            Config::from_apikey("app_key", "app_secret", "opaque")
                .http_url(format!("http://{addr}")),
        );
        let ctx = AssetContext::new(config);
        let full = ctx
            .deposits(GetDepositsOptions::new().size(50))
            .await
            .expect("deposits");
        assert_eq!(full.items.len(), 20, "a full capped page");
        assert!(
            full.has_more,
            "size 50 is sent as 20, so 20 records is a full page"
        );
        let short = ctx.deposits(None).await.expect("deposits");
        assert!(!short.has_more, "15 of the default 20 is the last page");
        let lines = server.await.expect("server task");
        assert!(
            lines[0].contains("size=20"),
            "capped size sent: {}",
            lines[0]
        );
        assert!(
            lines[1].contains("size=20"),
            "default size sent: {}",
            lines[1]
        );
        assert!(
            !lines[1].contains("account_channel"),
            "an opaque token sends no channel: {}",
            lines[1]
        );
    }

    /// The account routes are served by the AP data center only.
    #[tokio::test(flavor = "multi_thread")]
    async fn account_endpoints_reject_us_token() {
        let config = Arc::new(Config::from_apikey("us_app_key", "app_secret", "us_token"));
        let ctx = AssetContext::new(config);
        match ctx.bank_cards().await {
            Err(Error::HttpClient(HttpClientError::DcRegionRestricted {
                path, required, ..
            })) => {
                assert_eq!(path, core::BANK_CARDS_PATH, "restricted path mismatch");
                assert_eq!(required, DcRegion::Ap, "expected AP-only restriction");
            }
            other => panic!("expected DcRegionRestricted, got {other:?}"),
        }
        match ctx.deposits(None).await {
            Err(Error::HttpClient(HttpClientError::DcRegionRestricted { path, .. })) => {
                assert_eq!(path, core::DEPOSITS_PATH, "restricted path mismatch");
            }
            other => panic!("expected DcRegionRestricted, got {other:?}"),
        }
    }
}
