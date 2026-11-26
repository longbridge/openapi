use std::sync::Arc;

use longbridge::asset::AssetContext;
use pyo3::{prelude::*, types::PyType};

use crate::{
    asset::{
        context::{
            coupons_options, deposits_options, reward_records_options, stock_transfers_options,
            withdrawals_options,
        },
        types::{
            BankCard, CouponOrder, CouponOrderType, CouponsResponse, DepositState,
            DepositsResponse, EddaApplication, GetStatementListResponse, GetStatementResponse,
            RewardRecordsResponse, RewardType, StatementType, StockTransferInResponse,
            StockTransferInState, StockTransferOut, StockTransferOutState, WithdrawalsResponse,
        },
    },
    config::Config,
    error::ErrorNewType,
    time::PyInstantWrapper,
};

/// Async asset context.
#[pyclass]
pub(crate) struct AsyncAssetContext {
    ctx: Arc<AssetContext>,
}

#[pymethods]
impl AsyncAssetContext {
    /// Create an async asset context.
    #[classmethod]
    fn create(_cls: &Bound<PyType>, config: &Config) -> Self {
        AsyncAssetContext {
            ctx: Arc::new(AssetContext::new(Arc::new(config.0.clone()))),
        }
    }

    /// Get statement data list. Returns awaitable.
    #[pyo3(signature = (statement_type, start_date = 1, limit = 20))]
    fn statements(
        &self,
        py: Python<'_>,
        statement_type: StatementType,
        start_date: i32,
        limit: i32,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let opts = longbridge::asset::GetStatementListOptions::new(statement_type.into())
                .page(start_date)
                .page_size(limit);
            let resp = ctx.statements(opts).await.map_err(ErrorNewType)?;
            Ok(GetStatementListResponse::from(resp))
        })
        .map(|b| b.unbind())
    }

    /// Get statement data download URL. Returns awaitable.
    fn statement_download_url(&self, py: Python<'_>, file_key: String) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let opts = longbridge::asset::GetStatementOptions::new(file_key);
            let resp = ctx
                .statement_download_url(opts)
                .await
                .map_err(ErrorNewType)?;
            Ok(GetStatementResponse::from(resp))
        })
        .map(|b| b.unbind())
    }

    /// Get the linked bank cards. Returns awaitable.
    ///
    /// AP accounts only: US data-center accounts get a region error.
    fn bank_cards(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.bank_cards()
                .await
                .map_err(ErrorNewType)?
                .into_iter()
                .map(|x| -> PyResult<BankCard> { x.try_into() })
                .collect::<PyResult<Vec<BankCard>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get the eDDA authorization applications of a bank card in one
    /// currency. Returns awaitable.
    fn edda_applications(
        &self,
        py: Python<'_>,
        card_id: String,
        currency: String,
    ) -> PyResult<Py<PyAny>> {
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.edda_applications(card_id, currency)
                .await
                .map_err(ErrorNewType)?
                .into_iter()
                .map(|x| -> PyResult<EddaApplication> { x.try_into() })
                .collect::<PyResult<Vec<EddaApplication>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get stock transfer-in applications. Returns awaitable.
    #[pyo3(signature = (market = None, state = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn stock_transfers_in(
        &self,
        py: Python<'_>,
        market: Option<String>,
        state: Option<StockTransferInState>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let opts = stock_transfers_options(market, state, start, end, page, size);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v: StockTransferInResponse = ctx
                .stock_transfers_in(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(v)
        })
        .map(|b| b.unbind())
    }

    /// Get stock transfer-out applications. Returns awaitable.
    #[pyo3(signature = (market = None, state = None, start = None, end = None, page = None, size = None))]
    #[allow(clippy::too_many_arguments)]
    fn stock_transfers_out(
        &self,
        py: Python<'_>,
        market: Option<String>,
        state: Option<StockTransferOutState>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
        page: Option<i32>,
        size: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let opts = stock_transfers_options(market, state, start, end, page, size);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.stock_transfers_out(opts)
                .await
                .map_err(ErrorNewType)?
                .into_iter()
                .map(|x| -> PyResult<StockTransferOut> { x.try_into() })
                .collect::<PyResult<Vec<StockTransferOut>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get reward records from the reward center. Returns awaitable.
    ///
    /// The route has no date filter: page through the records and filter on
    /// their times.
    #[pyo3(signature = (page = None, page_size = None, reward_type = None))]
    fn reward_records(
        &self,
        py: Python<'_>,
        page: Option<i32>,
        page_size: Option<i32>,
        reward_type: Option<RewardType>,
    ) -> PyResult<Py<PyAny>> {
        let opts = reward_records_options(page, page_size, reward_type);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v: RewardRecordsResponse = ctx
                .reward_records(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(v)
        })
        .map(|b| b.unbind())
    }

    /// Get coupons (reward cards). Returns awaitable.
    #[pyo3(signature = (page = None, page_size = None, expired = false, card_type = None))]
    fn coupons(
        &self,
        py: Python<'_>,
        page: Option<i32>,
        page_size: Option<i32>,
        expired: bool,
        card_type: Option<String>,
    ) -> PyResult<Py<PyAny>> {
        let opts = coupons_options(page, page_size, expired, card_type);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v: CouponsResponse = ctx.coupons(opts).await.map_err(ErrorNewType)?.try_into()?;
            Ok(v)
        })
        .map(|b| b.unbind())
    }

    /// Get the orders a coupon was applied to. Returns awaitable.
    fn coupon_orders(
        &self,
        py: Python<'_>,
        card_id: String,
        order_type: CouponOrderType,
    ) -> PyResult<Py<PyAny>> {
        let order_type = order_type.into();
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            ctx.coupon_orders(card_id, order_type)
                .await
                .map_err(ErrorNewType)?
                .into_iter()
                .map(|x| -> PyResult<CouponOrder> { x.try_into() })
                .collect::<PyResult<Vec<CouponOrder>>>()
        })
        .map(|b| b.unbind())
    }

    /// Get one page of deposit records. Returns awaitable.
    #[pyo3(signature = (page = None, size = None, states = None, currencies = None, start = None, end = None))]
    #[allow(clippy::too_many_arguments)]
    fn deposits(
        &self,
        py: Python<'_>,
        page: Option<i32>,
        size: Option<i32>,
        states: Option<Vec<DepositState>>,
        currencies: Option<Vec<String>>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
    ) -> PyResult<Py<PyAny>> {
        let opts = deposits_options(page, size, states, currencies, start, end);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v: DepositsResponse = ctx.deposits(opts).await.map_err(ErrorNewType)?.try_into()?;
            Ok(v)
        })
        .map(|b| b.unbind())
    }

    /// Get withdrawal records. Returns awaitable.
    #[pyo3(signature = (page = None, size = None, start = None, end = None))]
    fn withdrawals(
        &self,
        py: Python<'_>,
        page: Option<i32>,
        size: Option<i32>,
        start: Option<PyInstantWrapper>,
        end: Option<PyInstantWrapper>,
    ) -> PyResult<Py<PyAny>> {
        let opts = withdrawals_options(page, size, start, end);
        let ctx = self.ctx.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let v: WithdrawalsResponse = ctx
                .withdrawals(opts)
                .await
                .map_err(ErrorNewType)?
                .try_into()?;
            Ok(v)
        })
        .map(|b| b.unbind())
    }
}
