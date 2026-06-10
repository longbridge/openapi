use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    fundamental::types::*,
    utils::counter::{counter_id_to_symbol, symbol_to_counter_id},
};

/// Convert a Unix-seconds string to RFC 3339.
fn unix_secs_str_to_rfc3339(s: &str) -> String {
    s.parse::<i64>()
        .ok()
        .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok())
        .map(|dt| {
            use time::format_description::well_known::Rfc3339;
            dt.format(&Rfc3339).unwrap_or_default()
        })
        .unwrap_or_else(|| s.to_string())
}

struct InnerFundamentalContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerFundamentalContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("fundamental context dropped");
        });
    }
}

/// Fundamental data context — financial reports, analyst ratings, dividends,
/// valuation, company overview and more.
#[derive(Clone)]
pub struct FundamentalContext(Arc<InnerFundamentalContext>);

impl FundamentalContext {
    /// Create a [`FundamentalContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("fundamental");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating fundamental context");
        });
        let ctx = Self(Arc::new(InnerFundamentalContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("fundamental context created");
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

    // ── financial_report ─────────────────────────────────────────

    /// Get financial reports for a security.
    ///
    /// Path: `GET /v1/quote/financial-reports`
    pub async fn financial_report(
        &self,
        symbol: impl Into<String>,
        kind: FinancialReportKind,
        period: Option<FinancialReportPeriod>,
    ) -> Result<FinancialReports> {
        let kind_str = match kind {
            FinancialReportKind::IncomeStatement => "IS",
            FinancialReportKind::BalanceSheet => "BS",
            FinancialReportKind::CashFlow => "CF",
            FinancialReportKind::All => "ALL",
        };
        let period_str = period.map(|p| match p {
            FinancialReportPeriod::Annual => "af",
            FinancialReportPeriod::SemiAnnual => "saf",
            FinancialReportPeriod::Q1 => "q1",
            FinancialReportPeriod::Q2 => "q2",
            FinancialReportPeriod::Q3 => "q3",
            FinancialReportPeriod::QuarterlyFull => "qf",
            FinancialReportPeriod::ThreeQ => "3q",
        });
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            kind: &'static str,
            #[serde(skip_serializing_if = "Option::is_none")]
            report: Option<&'static str>,
        }
        self.get(
            "/v1/quote/financial-reports",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                kind: kind_str,
                report: period_str,
            },
        )
        .await
    }

    // ── institution_rating ────────────────────────────────────────

    /// Get analyst ratings for a security (combines latest + historical).
    ///
    /// Path: `GET /v1/quote/institution-rating-latest` +
    ///       `GET /v1/quote/institution-ratings`
    pub async fn institution_rating(&self, symbol: impl Into<String>) -> Result<InstitutionRating> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        let cid = symbol_to_counter_id(&symbol.into());
        let q = Query { counter_id: cid };
        let (latest, summary) = tokio::join!(
            self.get::<InstitutionRatingLatest, _>(
                "/v1/quote/institution-rating-latest",
                Query {
                    counter_id: q.counter_id.clone()
                }
            ),
            self.get::<InstitutionRatingSummary, _>(
                "/v1/quote/institution-ratings",
                Query {
                    counter_id: q.counter_id.clone()
                }
            ),
        );
        Ok(InstitutionRating {
            latest: latest?,
            summary: summary?,
        })
    }

    /// Get historical analyst rating details for a security.
    ///
    /// Path: `GET /v1/quote/institution-ratings/detail`
    pub async fn institution_rating_detail(
        &self,
        symbol: impl Into<String>,
    ) -> Result<InstitutionRatingDetail> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/institution-ratings/detail",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── dividend ──────────────────────────────────────────────────

    /// Get dividend history for a security.
    ///
    /// Path: `GET /v1/quote/dividends`
    pub async fn dividend(&self, symbol: impl Into<String>) -> Result<DividendList> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/dividends",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get detailed dividend information for a security.
    ///
    /// Path: `GET /v1/quote/dividends/details`
    pub async fn dividend_detail(&self, symbol: impl Into<String>) -> Result<DividendList> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/dividends/details",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── forecast_eps ──────────────────────────────────────────────

    /// Get EPS forecasts for a security.
    ///
    /// Path: `GET /v1/quote/forecast-eps`
    pub async fn forecast_eps(&self, symbol: impl Into<String>) -> Result<ForecastEps> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/forecast-eps",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── consensus ─────────────────────────────────────────────────

    /// Get financial consensus estimates for a security.
    ///
    /// Path: `GET /v1/quote/financial-consensus-detail`
    pub async fn consensus(&self, symbol: impl Into<String>) -> Result<FinancialConsensus> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/financial-consensus-detail",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── valuation ─────────────────────────────────────────────────

    /// Get valuation metrics (PE/PB/PS/dividend yield) for a security.
    ///
    /// Path: `GET /v1/quote/valuation`
    pub async fn valuation(&self, symbol: impl Into<String>) -> Result<ValuationData> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            indicator: &'static str,
            range: &'static str,
        }
        self.get(
            "/v1/quote/valuation",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                indicator: "pe",
                range: "1",
            },
        )
        .await
    }

    /// Get historical valuation data for a security.
    ///
    /// Path: `GET /v1/quote/valuation/detail`
    pub async fn valuation_history(
        &self,
        symbol: impl Into<String>,
    ) -> Result<ValuationHistoryResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/valuation/detail",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── industry_valuation ────────────────────────────────────────

    /// Get valuation comparison against industry peers.
    ///
    /// Path: `GET /v1/quote/industry-valuation-comparison`
    pub async fn industry_valuation(
        &self,
        symbol: impl Into<String>,
    ) -> Result<IndustryValuationList> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/industry-valuation-comparison",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get valuation distribution within the industry.
    ///
    /// Path: `GET /v1/quote/industry-valuation-distribution`
    pub async fn industry_valuation_dist(
        &self,
        symbol: impl Into<String>,
    ) -> Result<IndustryValuationDist> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/industry-valuation-distribution",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── company ───────────────────────────────────────────────────

    /// Get company overview information.
    ///
    /// Path: `GET /v1/quote/comp-overview`
    pub async fn company(&self, symbol: impl Into<String>) -> Result<CompanyOverview> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/comp-overview",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── executive ─────────────────────────────────────────────────

    /// Get executive and board member information.
    ///
    /// Path: `GET /v1/quote/company-professionals`
    pub async fn executive(&self, symbol: impl Into<String>) -> Result<ExecutiveList> {
        #[derive(Serialize)]
        struct Query {
            counter_ids: String,
        }
        self.get(
            "/v1/quote/company-professionals",
            Query {
                counter_ids: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── shareholder ───────────────────────────────────────────────

    /// Get major shareholders for a security.
    ///
    /// Path: `GET /v1/quote/shareholders`
    pub async fn shareholder(&self, symbol: impl Into<String>) -> Result<ShareholderList> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/shareholders",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── fund_holder ───────────────────────────────────────────────

    /// Get funds and ETFs that hold a security.
    ///
    /// Path: `GET /v1/quote/fund-holders`
    pub async fn fund_holder(&self, symbol: impl Into<String>) -> Result<FundHolders> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/fund-holders",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── corp_action ───────────────────────────────────────────────

    /// Get corporate actions (dividends, splits, buybacks, etc.).
    ///
    /// Path: `GET /v1/quote/company-act`
    pub async fn corp_action(&self, symbol: impl Into<String>) -> Result<CorpActions> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            req_type: &'static str,
            version: &'static str,
        }
        self.get(
            "/v1/quote/company-act",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                req_type: "1",
                version: "3",
            },
        )
        .await
    }

    // ── invest_relation ───────────────────────────────────────────

    /// Get investor relations / investment holdings.
    ///
    /// Path: `GET /v1/quote/invest-relations`
    pub async fn invest_relation(&self, symbol: impl Into<String>) -> Result<InvestRelations> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            count: &'static str,
        }
        self.get(
            "/v1/quote/invest-relations",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                count: "0",
            },
        )
        .await
    }

    // ── operating ─────────────────────────────────────────────────

    /// Get operating metrics and financial report summaries.
    ///
    /// Path: `GET /v1/quote/operatings`
    pub async fn operating(&self, symbol: impl Into<String>) -> Result<OperatingList> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/operatings",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── buyback ───────────────────────────────────────────────────

    /// Get buyback data for a security.
    ///
    /// Path: `GET /v1/quote/buy-backs`
    pub async fn buyback(&self, symbol: impl Into<String>) -> Result<BuybackData> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/buy-backs",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── ratings ───────────────────────────────────────────────────

    /// Get stock ratings for a security.
    ///
    /// Path: `GET /v1/quote/ratings`
    pub async fn ratings(&self, symbol: impl Into<String>) -> Result<StockRatings> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/ratings",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── business_segments ────────────────────────────────────────

    /// Get the latest business segment breakdown for a security.
    ///
    /// Path: `GET /v1/quote/fundamentals/business-segments`
    pub async fn business_segments(&self, symbol: impl Into<String>) -> Result<BusinessSegments> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/fundamentals/business-segments",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get historical business segment breakdowns for a security.
    ///
    /// Path: `GET /v1/quote/fundamentals/business-segments/history`
    pub async fn business_segments_history(
        &self,
        symbol: impl Into<String>,
        report: Option<&'static str>,
        cate: Option<String>,
    ) -> Result<BusinessSegmentsHistory> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            report: Option<&'static str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            cate: Option<String>,
        }
        self.get(
            "/v1/quote/fundamentals/business-segments/history",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                report,
                cate,
            },
        )
        .await
    }

    // ── shareholder_top ───────────────────────────────────────────

    /// Get a ranked list of top shareholders for a security.
    ///
    /// Path: `GET /v1/quote/shareholders/top`
    pub async fn shareholder_top(
        &self,
        symbol: impl Into<String>,
    ) -> Result<ShareholderTopResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/shareholders/top",
                Query {
                    counter_id: symbol_to_counter_id(&symbol.into()),
                },
            )
            .await?;
        Ok(ShareholderTopResponse { data: raw })
    }

    // ── institution_rating_views ──────────────────────────────────

    /// Get historical institutional rating view time-series for a security.
    ///
    /// Path: `GET /v1/quote/ratings/institutional`
    pub async fn institution_rating_views(
        &self,
        symbol: impl Into<String>,
    ) -> Result<InstitutionRatingViews> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/ratings/institutional",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── shareholder_detail ────────────────────────────────────────

    /// Get holding history and detail for one shareholder object.
    ///
    /// Path: `GET /v1/quote/shareholders/holding`
    pub async fn shareholder_detail(
        &self,
        symbol: impl Into<String>,
        object_id: i64,
    ) -> Result<ShareholderDetailResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            object_id: String,
        }
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/shareholders/holding",
                Query {
                    counter_id: symbol_to_counter_id(&symbol.into()),
                    object_id: object_id.to_string(),
                },
            )
            .await?;
        Ok(ShareholderDetailResponse { data: raw })
    }

    // ── industry_rank ─────────────────────────────────────────────

    /// Get industry rank for a market.
    ///
    /// Path: `GET /v1/quote/industry/rank`
    pub async fn industry_rank(
        &self,
        market: impl Into<String>,
        indicator: impl Into<String>,
        sort_type: impl Into<String>,
        limit: u32,
    ) -> Result<IndustryRankResponse> {
        #[derive(Serialize)]
        struct Query {
            market: String,
            indicator: String,
            sort_type: String,
            limit: u32,
        }
        self.get(
            "/v1/quote/industry/rank",
            Query {
                market: market.into(),
                indicator: indicator.into(),
                sort_type: sort_type.into(),
                limit,
            },
        )
        .await
    }

    // ── industry_peers ────────────────────────────────────────────

    /// Get the industry peer chain for a security or industry.
    ///
    /// Path: `GET /v1/quote/industries/peers`
    pub async fn industry_peers(
        &self,
        counter_id: impl Into<String>,
        market: impl Into<String>,
        industry_id: Option<String>,
    ) -> Result<IndustryPeersResponse> {
        let raw = counter_id.into();
        let cid = if raw.contains('/') {
            raw
        } else {
            symbol_to_counter_id(&raw)
        };
        #[derive(Serialize)]
        struct Query {
            #[serde(rename = "type")]
            kind: &'static str,
            market: String,
            industry_id: String,
            counter_id: String,
        }
        self.get(
            "/v1/quote/industries/peers",
            Query {
                kind: "1",
                market: market.into(),
                industry_id: industry_id.unwrap_or_default(),
                counter_id: cid,
            },
        )
        .await
    }

    // ── financial_report_snapshot ─────────────────────────────────

    /// Get a financial report snapshot (earnings snapshot) for a security.
    ///
    /// Path: `GET /v1/quote/financials/earnings-snapshot`
    pub async fn financial_report_snapshot(
        &self,
        symbol: impl Into<String>,
        report: Option<&'static str>,
        fiscal_year: Option<i32>,
        fiscal_period: Option<&'static str>,
    ) -> Result<FinancialReportSnapshot> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            report: Option<&'static str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fiscal_year: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            fiscal_period: Option<&'static str>,
        }
        self.get(
            "/v1/quote/financials/earnings-snapshot",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
                report,
                fiscal_year,
                fiscal_period,
            },
        )
        .await
    }

    // ── valuation_comparison ──────────────────────────────────────

    /// Get valuation comparison between a security and optional peers.
    ///
    /// Path: `GET /v1/quote/compare/valuation`
    pub async fn valuation_comparison(
        &self,
        symbol: impl Into<String>,
        currency: impl Into<String>,
        comparison_symbols: Option<Vec<String>>,
    ) -> Result<ValuationComparisonResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
            currency: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            comparison_counter_ids: Option<String>,
        }
        let comparison_counter_ids = comparison_symbols.map(|syms| {
            let ids: Vec<String> = syms.iter().map(|s| symbol_to_counter_id(s)).collect();
            serde_json::to_string(&ids).unwrap_or_default()
        });
        let raw: serde_json::Value = self
            .get(
                "/v1/quote/compare/valuation",
                Query {
                    counter_id: symbol_to_counter_id(&symbol.into()),
                    currency: currency.into(),
                    comparison_counter_ids,
                },
            )
            .await?;
        let list = raw["list"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                let history = item["history"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|h| ValuationHistoryPoint {
                        date: unix_secs_str_to_rfc3339(h["date"].as_str().unwrap_or("")),
                        pe: h["pe"].as_str().unwrap_or("").to_string(),
                        pb: h["pb"].as_str().unwrap_or("").to_string(),
                        ps: h["ps"].as_str().unwrap_or("").to_string(),
                    })
                    .collect();
                ValuationComparisonItem {
                    symbol: counter_id_to_symbol(item["counter_id"].as_str().unwrap_or("")),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    currency: item["currency"].as_str().unwrap_or("").to_string(),
                    market_value: item["market_value"].as_str().unwrap_or("").to_string(),
                    price_close: item["price_close"].as_str().unwrap_or("").to_string(),
                    pe: item["pe"].as_str().unwrap_or("").to_string(),
                    pb: item["pb"].as_str().unwrap_or("").to_string(),
                    ps: item["ps"].as_str().unwrap_or("").to_string(),
                    roe: item["roe"].as_str().unwrap_or("").to_string(),
                    eps: item["eps"].as_str().unwrap_or("").to_string(),
                    bps: item["bps"].as_str().unwrap_or("").to_string(),
                    dps: item["dps"].as_str().unwrap_or("").to_string(),
                    div_yld: item["div_yld"].as_str().unwrap_or("").to_string(),
                    assets: item["assets"].as_str().unwrap_or("").to_string(),
                    history,
                }
            })
            .collect();
        Ok(ValuationComparisonResponse { list })
    }

    // ── etf_asset_allocation ─────────────────────────────────────

    /// Get ETF asset allocation (holdings / regional / asset class /
    /// industry).
    ///
    /// Path: `GET /v1/quote/etf-asset-allocation`
    pub async fn etf_asset_allocation(
        &self,
        symbol: impl Into<String>,
    ) -> Result<AssetAllocationResponse> {
        #[derive(Serialize)]
        struct Query {
            counter_id: String,
        }
        self.get(
            "/v1/quote/etf-asset-allocation",
            Query {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    // ── macrodata ────────────────────────────────────────────────

    /// List macroeconomic indicators.
    ///
    /// Pass `country` to filter by country code (e.g. `MacrodataCountry::UnitedStates`).
    ///
    /// Path: `GET /v1/quote/macrodata`
    pub async fn macrodata_indicators(
        &self,
        country: Option<MacrodataCountry>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacrodataIndicatorListResponse> {
        #[derive(Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            country: Option<MacrodataCountry>,
            #[serde(skip_serializing_if = "Option::is_none")]
            offset: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<i32>,
        }
        self.get(
            "/v1/quote/macrodata",
            Query {
                country,
                offset,
                limit,
            },
        )
        .await
    }

    /// Get historical data for a macroeconomic indicator.
    ///
    /// `start_date` and `end_date` are date strings in `"YYYY-MM-DD"` format.
    /// `start_date` is sent as `YYYY-MM-DDT00:00:00Z`; `end_date` is sent as
    /// `YYYY-MM-DDT23:59:59Z`.
    ///
    /// Path: `GET /v1/quote/macrodata/{indicator_code}`
    pub async fn macrodata(
        &self,
        indicator_code: impl Into<String>,
        start_date: Option<impl Into<String>>,
        end_date: Option<impl Into<String>>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<MacrodataResponse> {
        #[derive(Serialize)]
        struct Query {
            #[serde(skip_serializing_if = "Option::is_none")]
            start_time: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_time: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            offset: Option<i32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<i32>,
        }
        let path = format!("/v1/quote/macrodata/{}", indicator_code.into());
        Ok(self
            .0
            .http_cli
            .request(Method::GET, path)
            .query_params(Query {
                start_time: start_date.map(|d| format!("{}T00:00:00Z", d.into())),
                end_time: end_date.map(|d| format!("{}T23:59:59Z", d.into())),
                offset,
                limit,
            })
            .response::<Json<MacrodataResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
