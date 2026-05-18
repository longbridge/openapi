use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, fundamental::types::*, utils::counter::symbol_to_counter_id};

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

    // ── industry_rank ─────────────────────────────────────────────

    /// Get industry rank for a market.
    ///
    /// Path: `GET /v1/quote/industry/rank`
    ///
    /// `indicator` is a numeric string `"0"`–`"7"`;
    /// `sort_type` is `"0"` (ascending) or `"1"` (descending).
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
    ///
    /// `counter_id` may be a regular symbol (e.g. `"AAPL.US"`) or an industry
    /// counter ID (e.g. `"BK/US/123"`) — pass it through as-is if it already
    /// contains a `/`.
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
}
