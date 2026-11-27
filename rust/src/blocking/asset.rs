use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    asset::{
        AssetContext, BankCard, CouponOrder, CouponOrderType, CouponsResponse, DepositsResponse,
        EddaApplication, GetCouponsOptions, GetDepositsOptions, GetRewardRecordsOptions,
        GetStatementListOptions, GetStatementListResponse, GetStatementOptions,
        GetStatementResponse, GetStockTransfersInOptions, GetStockTransfersOutOptions,
        GetWithdrawalsOptions, RewardRecordsResponse, StockTransferInResponse, StockTransferOut,
        WithdrawalsResponse,
    },
    blocking::runtime::BlockingRuntime,
};

/// Blocking asset context
pub struct AssetContextSync {
    rt: BlockingRuntime<AssetContext>,
}

impl AssetContextSync {
    /// Create a `AssetContextSync`
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = AssetContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get statement data list
    pub fn statements(&self, options: GetStatementListOptions) -> Result<GetStatementListResponse> {
        self.rt
            .call(move |ctx| async move { ctx.statements(options).await })
    }

    /// Get statement data download url
    pub fn statement_download_url(
        &self,
        options: GetStatementOptions,
    ) -> Result<GetStatementResponse> {
        self.rt
            .call(move |ctx| async move { ctx.statement_download_url(options).await })
    }

    /// Get the linked bank cards, with their verification, binding and
    /// authorization status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{Config, blocking::AssetContextSync, oauth::OAuthBuilder};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let oauth =
    ///     OAuthBuilder::new("your-client-id").build_blocking(|url| println!("Visit: {url}"))?;
    /// let config = Arc::new(Config::from_oauth(oauth));
    /// let ctx = AssetContextSync::new(config)?;
    ///
    /// for card in ctx.bank_cards()? {
    ///     println!("{} {:?}", card.bank, card.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn bank_cards(&self) -> Result<Vec<BankCard>> {
        self.rt
            .call(move |ctx| async move { ctx.bank_cards().await })
    }

    /// Get the eDDA authorization applications of a bank card in one
    /// currency
    pub fn edda_applications(
        &self,
        card_id: impl Into<String>,
        currency: impl Into<String>,
    ) -> Result<Vec<EddaApplication>> {
        let card_id = card_id.into();
        let currency = currency.into();
        self.rt
            .call(move |ctx| async move { ctx.edda_applications(card_id, currency).await })
    }

    /// Get stock transfer-in applications (another broker → this account)
    pub fn stock_transfers_in(
        &self,
        options: impl Into<Option<GetStockTransfersInOptions>>,
    ) -> Result<StockTransferInResponse> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.stock_transfers_in(options).await })
    }

    /// Get stock transfer-out applications (this account → another broker)
    pub fn stock_transfers_out(
        &self,
        options: impl Into<Option<GetStockTransfersOutOptions>>,
    ) -> Result<Vec<StockTransferOut>> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.stock_transfers_out(options).await })
    }

    /// Get reward records from the reward center
    ///
    /// The route has no date filter: to find rewards in a time range, page
    /// through the records and filter on their times.
    pub fn reward_records(
        &self,
        options: impl Into<Option<GetRewardRecordsOptions>>,
    ) -> Result<RewardRecordsResponse> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.reward_records(options).await })
    }

    /// Get coupons (reward cards)
    pub fn coupons(
        &self,
        options: impl Into<Option<GetCouponsOptions>>,
    ) -> Result<CouponsResponse> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.coupons(options).await })
    }

    /// Get the orders a coupon was applied to
    pub fn coupon_orders(
        &self,
        card_id: impl Into<String>,
        order_type: CouponOrderType,
    ) -> Result<Vec<CouponOrder>> {
        let card_id = card_id.into();
        self.rt
            .call(move |ctx| async move { ctx.coupon_orders(card_id, order_type).await })
    }

    /// Get one page of deposit records (see
    /// [`crate::AssetContext::deposits`])
    pub fn deposits(
        &self,
        options: impl Into<Option<GetDepositsOptions>>,
    ) -> Result<DepositsResponse> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.deposits(options).await })
    }

    /// Get withdrawal records
    pub fn withdrawals(
        &self,
        options: impl Into<Option<GetWithdrawalsOptions>>,
    ) -> Result<WithdrawalsResponse> {
        let options = options.into();
        self.rt
            .call(move |ctx| async move { ctx.withdrawals(options).await })
    }
}
