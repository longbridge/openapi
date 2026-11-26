use std::sync::Arc;

use longbridge_httpcli::{DcRegion, HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    asset::{
        BankCard, BankCardsResponse, CouponOrder, CouponOrderType, CouponOrdersResponse,
        CouponsResponse, Deposit, DepositsResponse, EddaApplication, EddaApplicationsResponse,
        GetCouponsOptions, GetDepositsOptions, GetRewardRecordsOptions, GetStatementListOptions,
        GetStatementListResponse, GetStatementOptions, GetStatementResponse,
        GetStockTransfersInOptions, GetStockTransfersOutOptions, GetWithdrawalsOptions,
        RewardRecordsResponse, StockTransferInResponse, StockTransferOut, StockTransferOutResponse,
        WithdrawalsResponse, core,
        requests::{GetCouponOrdersQuery, GetEddaApplicationsQuery},
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
    /// region error instead of reaching a server that rejects it.
    async fn get_account<R, Q>(&self, path: &'static str, query: Q) -> Result<R>
    where
        R: DeserializeOwned + Send + Sync + 'static,
        Q: Serialize + Send + Sync,
    {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .dc_restrict(DcRegion::Ap)
            .query_params(query)
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
            .get_account::<CouponOrdersResponse, _>(core::COUPON_ORDERS_PATH, query)
            .await?
            .orders)
    }

    /// Get deposit records
    ///
    /// The server's `total` follows the page size rather than the record
    /// count, so it is not returned; a full page means there may be more.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// Path: `GET /v1/account/deposits`
    pub async fn deposits(
        &self,
        options: impl Into<Option<GetDepositsOptions>>,
    ) -> Result<Vec<Deposit>> {
        Ok(self
            .get_account::<DepositsResponse, _>(
                core::DEPOSITS_PATH,
                options.into().unwrap_or_default(),
            )
            .await?
            .items)
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
