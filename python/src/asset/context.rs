use std::sync::Arc;

use longbridge::{asset::StockTransferStateFilter, blocking::AssetContextSync};
use pyo3::prelude::*;

use crate::{
    asset::types::{
        BankCard, CouponOrder, CouponOrderType, CouponsResponse, Deposit, DepositState,
        EddaApplication, GetStatementListResponse, GetStatementResponse, RewardRecordsResponse,
        RewardType, StatementType, StockTransferInResponse, StockTransferInState, StockTransferOut,
        StockTransferOutState, WithdrawalsResponse,
    },
    config::Config,
    error::ErrorNewType,
    time::PyInstantWrapper,
};

/// Builds the stock transfer list options from keyword arguments.
pub(crate) fn stock_transfers_options<S, C>(
    market: Option<String>,
    state: Option<S>,
    start: Option<PyInstantWrapper>,
    end: Option<PyInstantWrapper>,
    page: Option<i32>,
    size: Option<i32>,
) -> longbridge::asset::GetStockTransfersOptions<C>
where
    S: Into<C>,
    C: StockTransferStateFilter,
{
    let mut opts = longbridge::asset::GetStockTransfersOptions::new();
    if let Some(market) = market {
        opts = opts.market(market);
    }
    if let Some(state) = state {
        opts = opts.state(state.into());
    }
    if let Some(start) = start {
        opts = opts.start(start.0);
    }
    if let Some(end) = end {
        opts = opts.end(end.0);
    }
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(size) = size {
        opts = opts.size(size);
    }
    opts
}

/// Builds the reward record list options from keyword arguments.
pub(crate) fn reward_records_options(
    page: Option<i32>,
    page_size: Option<i32>,
    reward_type: Option<RewardType>,
) -> longbridge::asset::GetRewardRecordsOptions {
    let mut opts = longbridge::asset::GetRewardRecordsOptions::new();
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(page_size) = page_size {
        opts = opts.page_size(page_size);
    }
    if let Some(reward_type) = reward_type {
        opts = opts.reward_type(reward_type.into());
    }
    opts
}

/// Builds the coupon list options from keyword arguments.
pub(crate) fn coupons_options(
    page: Option<i32>,
    page_size: Option<i32>,
    expired: bool,
    card_type: Option<String>,
) -> longbridge::asset::GetCouponsOptions {
    let mut opts = longbridge::asset::GetCouponsOptions::new().expired(expired);
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(page_size) = page_size {
        opts = opts.page_size(page_size);
    }
    if let Some(card_type) = card_type {
        opts = opts.card_type(card_type);
    }
    opts
}

/// Builds the deposit list options from keyword arguments.
pub(crate) fn deposits_options(
    page: Option<i32>,
    size: Option<i32>,
    states: Option<Vec<DepositState>>,
    currencies: Option<Vec<String>>,
    start: Option<PyInstantWrapper>,
    end: Option<PyInstantWrapper>,
) -> longbridge::asset::GetDepositsOptions {
    let mut opts = longbridge::asset::GetDepositsOptions::new();
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(size) = size {
        opts = opts.size(size);
    }
    if let Some(states) = states {
        opts = opts.states(states.into_iter().map(Into::into));
    }
    if let Some(currencies) = currencies {
        opts = opts.currencies(currencies);
    }
    if let Some(start) = start {
        opts = opts.start(start.0);
    }
    if let Some(end) = end {
        opts = opts.end(end.0);
    }
    opts
}

/// Builds the withdrawal list options from keyword arguments.
pub(crate) fn withdrawals_options(
    page: Option<i32>,
    size: Option<i32>,
    start: Option<PyInstantWrapper>,
    end: Option<PyInstantWrapper>,
) -> longbridge::asset::GetWithdrawalsOptions {
    let mut opts = longbridge::asset::GetWithdrawalsOptions::new();
    if let Some(page) = page {
        opts = opts.page(page);
    }
    if let Some(size) = size {
        opts = opts.size(size);
    }
    if let Some(start) = start {
        opts = opts.start(start.0);
    }
    if let Some(end) = end {
        opts = opts.end(end.0);
    }
    opts
}

#[pyclass]
pub(crate) struct AssetContext {
    ctx: AssetContextSync,
}

#[pymethods]
impl AssetContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: AssetContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get statement data list
    #[pyo3(signature = (statement_type, start_date = 1, limit = 20))]
    pub fn statements(
        &self,
        statement_type: StatementType,
        start_date: i32,
        limit: i32,
    ) -> PyResult<GetStatementListResponse> {
        let opts = longbridge::asset::GetStatementListOptions::new(statement_type.into())
            .page(start_date)
            .page_size(limit);
        Ok(self.ctx.statements(opts).map_err(ErrorNewType)?.into())
    }

    /// Get statement data download URL
    pub fn statement_download_url(&self, file_key: String) -> PyResult<GetStatementResponse> {
        let opts = longbridge::asset::GetStatementOptions::new(file_key);
        Ok(self
            .ctx
            .statement_download_url(opts)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the linked bank cards, with their verification, binding and
    /// authorization status
    ///
    /// AP accounts only: US data-center accounts get a region error.
    fn bank_cards(&self) -> PyResult<Vec<BankCard>> {
        self.ctx
            .bank_cards()
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get the eDDA authorization applications of a bank card in one
    /// currency
    fn edda_applications(
        &self,
        card_id: String,
        currency: String,
    ) -> PyResult<Vec<EddaApplication>> {
        self.ctx
            .edda_applications(card_id, currency)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get stock transfer-in applications (another broker to this account)
    #[pyo3(signature = (market = None, state = None, start = None, end = None, page = None, size = None))]
    fn stock_transfers_in(
        &self,
        market: Option<String>,
        state: Option<StockTransferInState>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<StockTransferInResponse> {
        let opts = stock_transfers_options(market, state, start, end, page, size);
        self.ctx
            .stock_transfers_in(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get stock transfer-out applications (this account to another broker)
    #[pyo3(signature = (market = None, state = None, start = None, end = None, page = None, size = None))]
    fn stock_transfers_out(
        &self,
        market: Option<String>,
        state: Option<StockTransferOutState>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Vec<StockTransferOut>> {
        let opts = stock_transfers_options(market, state, start, end, page, size);
        self.ctx
            .stock_transfers_out(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get reward records from the reward center
    ///
    /// The route has no date filter: page through the records and filter on
    /// their times.
    #[pyo3(signature = (page = None, page_size = None, reward_type = None))]
    fn reward_records(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        reward_type: Option<RewardType>,
    ) -> PyResult<RewardRecordsResponse> {
        let opts = reward_records_options(page, page_size, reward_type);
        self.ctx
            .reward_records(opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Get coupons (reward cards)
    #[pyo3(signature = (page = None, page_size = None, expired = false, card_type = None))]
    fn coupons(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        expired: bool,
        card_type: Option<String>,
    ) -> PyResult<CouponsResponse> {
        let opts = coupons_options(page, page_size, expired, card_type);
        self.ctx.coupons(opts).map_err(ErrorNewType)?.try_into()
    }

    /// Get the orders a coupon was applied to
    fn coupon_orders(
        &self,
        card_id: String,
        order_type: CouponOrderType,
    ) -> PyResult<Vec<CouponOrder>> {
        self.ctx
            .coupon_orders(card_id, order_type.into())
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get deposit records
    #[pyo3(signature = (page = None, size = None, states = None, currencies = None, start = None, end = None))]
    fn deposits(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        states: Option<Vec<DepositState>>,
        currencies: Option<Vec<String>>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
    ) -> PyResult<Vec<Deposit>> {
        let opts = deposits_options(page, size, states, currencies, start, end);
        self.ctx
            .deposits(opts)
            .map_err(ErrorNewType)?
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }

    /// Get withdrawal records
    #[pyo3(signature = (page = None, size = None, start = None, end = None))]
    fn withdrawals(
        &self,
        page: Option<i32>,
        size: Option<i32>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
    ) -> PyResult<WithdrawalsResponse> {
        let opts = withdrawals_options(page, size, start, end);
        self.ctx.withdrawals(opts).map_err(ErrorNewType)?.try_into()
    }
}
