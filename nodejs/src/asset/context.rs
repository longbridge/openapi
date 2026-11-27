use std::sync::Arc;

use napi::Result;

use crate::{
    asset::{
        requests::{
            GetCouponsOptions, GetDepositsOptions, GetRewardRecordsOptions,
            GetStatementDownloadUrlRequest, GetStatementListRequest, GetStockTransfersInOptions,
            GetStockTransfersOutOptions, GetWithdrawalsOptions, StatementType,
        },
        types::{
            BankCard, CouponOrder, CouponOrderType, CouponsResponse, DepositsResponse,
            EddaApplication, GetStatementDownloadUrlResponse, GetStatementListResponse,
            RewardRecordsResponse, StockTransferInResponse, StockTransferOut, WithdrawalsResponse,
        },
    },
    config::Config,
    error::ErrorNewType,
};

/// Asset context
#[napi_derive::napi]
#[derive(Clone)]
pub struct AssetContext {
    ctx: longbridge::asset::AssetContext,
}

#[napi_derive::napi]
impl AssetContext {
    /// Create a new `AssetContext`
    #[napi]
    pub fn new(config: &Config) -> AssetContext {
        Self {
            ctx: longbridge::asset::AssetContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get statement data list
    #[napi]
    pub async fn statements(
        &self,
        req: Option<GetStatementListRequest>,
    ) -> Result<GetStatementListResponse> {
        let req = req.unwrap_or_default();
        let st = req.statement_type.unwrap_or(StatementType::Daily).into();
        let mut opts = longbridge::asset::GetStatementListOptions::new(st);
        if let Some(start_date) = req.start_date {
            opts = opts.page(start_date);
        }
        if let Some(limit) = req.limit {
            opts = opts.page_size(limit);
        }
        Ok(self
            .ctx
            .statements(opts)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get statement data download URL
    #[napi]
    pub async fn statement_download_url(
        &self,
        req: GetStatementDownloadUrlRequest,
    ) -> Result<GetStatementDownloadUrlResponse> {
        let opts = longbridge::asset::GetStatementOptions::new(req.file_key);
        Ok(self
            .ctx
            .statement_download_url(opts)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the linked bank cards, with their verification, binding and
    /// authorization (eDDA, bank securities account, FPS) status
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.bankCards();
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn bank_cards(&self) -> Result<Vec<BankCard>> {
        self.ctx
            .bank_cards()
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the eDDA authorization applications of a bank card in one
    /// currency
    ///
    /// `cardId` is `BankCard.id`; `currency` is e.g. `HKD` or `USD`. The card
    /// id must be all digits; any other value fails with an error before
    /// anything is sent.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.eddaApplications('12345', 'HKD');
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn edda_applications(
        &self,
        card_id: String,
        currency: String,
    ) -> Result<Vec<EddaApplication>> {
        self.ctx
            .edda_applications(card_id, currency)
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get stock transfer-in applications (another broker to this account)
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.stockTransfersIn({ market: 'HK' });
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn stock_transfers_in(
        &self,
        opts: Option<GetStockTransfersInOptions>,
    ) -> Result<StockTransferInResponse> {
        self.ctx
            .stock_transfers_in(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get stock transfer-out applications (this account to another broker)
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.stockTransfersOut();
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn stock_transfers_out(
        &self,
        opts: Option<GetStockTransfersOutOptions>,
    ) -> Result<Vec<StockTransferOut>> {
        self.ctx
            .stock_transfers_out(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get reward records from the reward center (cash, gifts, coupons, task
    /// rewards)
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// The route has no date filter: to find rewards in a time range, page
    /// through the records and filter on their times.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.rewardRecords({ pageSize: 20 });
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn reward_records(
        &self,
        opts: Option<GetRewardRecordsOptions>,
    ) -> Result<RewardRecordsResponse> {
        self.ctx
            .reward_records(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get coupons (reward cards)
    ///
    /// By default lists the coupons that have not expired (set `expired`).
    /// The pinned `topList` / `bottomList` are repeated on every page.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.coupons({ expired: false });
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn coupons(&self, opts: Option<GetCouponsOptions>) -> Result<CouponsResponse> {
        self.ctx
            .coupons(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get the orders a coupon was applied to
    ///
    /// `cardId` is `Coupon.cardId` (32 hex characters, not a bank card ID);
    /// any other value fails with an error before anything is sent.
    /// The gateway rate-limits this route tightly per member, so avoid
    /// calling it concurrently.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.couponOrders(couponId, CouponOrderType.Settled);
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn coupon_orders(
        &self,
        card_id: String,
        order_type: CouponOrderType,
    ) -> Result<Vec<CouponOrder>> {
        self.ctx
            .coupon_orders(card_id, order_type.into())
            .await
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get one page of deposit records
    ///
    /// The server's total follows the page size rather than the record
    /// count, so it is not returned; `hasMore` reports a full page instead.
    /// The page size is always sent: at most 20, default 20.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.deposits({ size: 20 });
    /// console.log(resp.items, resp.hasMore);
    /// ```
    #[napi]
    pub async fn deposits(&self, opts: Option<GetDepositsOptions>) -> Result<DepositsResponse> {
        self.ctx
            .deposits(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get withdrawal records
    ///
    /// `hasMore` reports a full page of the size sent (default 20, at most
    /// 50).
    ///
    /// AP accounts only: US data-center accounts get a region error.
    ///
    /// #### Example
    ///
    /// ```javascript
    /// const { OAuth, Config, AssetContext } = require('longbridge')
    ///
    /// const oauth = await OAuth.build('your-client-id', (_, url) => console.log('Visit:', url));
    /// const ctx = AssetContext.new(Config.fromOAuth(oauth));
    /// const resp = await ctx.withdrawals({ size: 20 });
    /// console.log(resp);
    /// ```
    #[napi]
    pub async fn withdrawals(
        &self,
        opts: Option<GetWithdrawalsOptions>,
    ) -> Result<WithdrawalsResponse> {
        self.ctx
            .withdrawals(opts.map(Into::into))
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }
}
