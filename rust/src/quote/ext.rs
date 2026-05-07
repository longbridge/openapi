//! Extensions to [`QuoteContext`] – new fundamental-data and market-data methods.

use serde::Serialize;

use crate::{
    Result,
    quote::{
        QuoteContext,
        extra_types::{
            AhPremiumKlinesOptions, BrokerHoldingOptions, CorporateActionsOptions,
            DividendsOptions, FinanceCalendarOptions, FinancialReportOptions,
            InstitutionRatingDetailOptions, OperatingDataOptions, ValuationHistoryOptions,
            ValuationOptions,
        },
        utils::symbol_to_counter_id,
    },
};

impl QuoteContext {
    // -----------------------------------------------------------------------
    // Domain A: Fundamental Data  (counter_id conversion)
    // -----------------------------------------------------------------------

    /// Get financial reports for a symbol.
    ///
    /// Path: GET /v1/quote/financial-reports
    pub async fn financial_report(
        &self,
        symbol: impl Into<String>,
        opts: FinancialReportOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            kind: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            report: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/financial-reports",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                kind: opts.kind,
                report: opts.report_type,
            },
        )
        .await
    }

    /// Get institution ratings for a symbol.
    ///
    /// Path: GET /v1/quote/institution-ratings
    pub async fn institution_ratings(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/institution-ratings",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get the latest institution rating for a symbol.
    ///
    /// Path: GET /v1/quote/institution-rating-latest
    pub async fn institution_rating_latest(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/institution-rating-latest",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get institution rating detail for a symbol.
    ///
    /// Path: GET /v1/quote/institution-ratings/detail
    pub async fn institution_rating_detail(
        &self,
        symbol: impl Into<String>,
        opts: InstitutionRatingDetailOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            page: Option<u32>,
            #[serde(skip_serializing_if = "Option::is_none")]
            page_size: Option<u32>,
        }
        self.http_get_json(
            "/v1/quote/institution-ratings/detail",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                page: opts.page,
                page_size: opts.page_size,
            },
        )
        .await
    }

    /// Get dividends for a symbol.
    ///
    /// Path: GET /v1/quote/dividends
    pub async fn dividends(
        &self,
        symbol: impl Into<String>,
        opts: DividendsOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_date: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/dividends",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                start_date: opts.start_date,
                end_date: opts.end_date,
            },
        )
        .await
    }

    /// Get detail for a specific dividend of a symbol.
    ///
    /// Path: GET /v1/quote/dividends/details
    pub async fn dividend_detail(
        &self,
        symbol: impl Into<String>,
        dividend_id: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            dividend_id: String,
        }
        self.http_get_json(
            "/v1/quote/dividends/details",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                dividend_id: dividend_id.into(),
            },
        )
        .await
    }

    /// Get EPS forecasts for a symbol.
    ///
    /// Path: GET /v1/quote/forecast-eps
    pub async fn forecast_eps(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/forecast-eps",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get financial consensus detail for a symbol.
    ///
    /// Path: GET /v1/quote/financial-consensus-detail
    pub async fn financial_consensus(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/financial-consensus-detail",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get valuation data for a symbol.
    ///
    /// Path: GET /v1/quote/valuation
    pub async fn valuation(
        &self,
        symbol: impl Into<String>,
        opts: ValuationOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/valuation",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                period: opts.period,
            },
        )
        .await
    }

    /// Get valuation history for a symbol.
    ///
    /// Path: GET /v1/quote/valuation/detail
    pub async fn valuation_history(
        &self,
        symbol: impl Into<String>,
        opts: ValuationHistoryOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            count: Option<u32>,
        }
        self.http_get_json(
            "/v1/quote/valuation/detail",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                period: opts.period,
                count: opts.count,
            },
        )
        .await
    }

    /// Get industry valuation comparison for a symbol.
    ///
    /// Path: GET /v1/quote/industry-valuation-comparison
    pub async fn industry_valuation(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/industry-valuation-comparison",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get industry valuation distribution for a symbol.
    ///
    /// Path: GET /v1/quote/industry-valuation-distribution
    pub async fn industry_valuation_distribution(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/industry-valuation-distribution",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get company overview for a symbol.
    ///
    /// Path: GET /v1/quote/comp-overview
    pub async fn company_overview(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/comp-overview",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get company executives for a symbol.
    ///
    /// Path: GET /v1/quote/company-professionals
    pub async fn company_executives(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/company-professionals",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get shareholders data for a symbol.
    ///
    /// Path: GET /v1/quote/shareholders
    pub async fn shareholders(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/shareholders",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get fund holders for a symbol.
    ///
    /// Path: GET /v1/quote/fund-holders
    pub async fn fund_holders(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/fund-holders",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get corporate actions for a symbol.
    ///
    /// Path: GET /v1/quote/company-act
    pub async fn corporate_actions(
        &self,
        symbol: impl Into<String>,
        opts: CorporateActionsOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            action_type: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/company-act",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                action_type: opts.action_type,
            },
        )
        .await
    }

    /// Get investor relations for a symbol.
    ///
    /// Path: GET /v1/quote/invest-relations
    pub async fn investor_relations(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
        }
        self.http_get_json(
            "/v1/quote/invest-relations",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
            },
        )
        .await
    }

    /// Get operating data for a symbol.
    ///
    /// Path: GET /v1/quote/operatings
    pub async fn operating_data(
        &self,
        symbol: impl Into<String>,
        opts: OperatingDataOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            counter_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/operatings",
            Request {
                counter_id: symbol_to_counter_id(&symbol.into()),
                period: opts.period,
            },
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Domain B: Market Data  (no counter_id conversion)
    // -----------------------------------------------------------------------

    /// Get market status.
    ///
    /// Path: GET /v1/quote/market-status
    pub async fn market_status(&self, market: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            market: String,
        }
        self.http_get_json(
            "/v1/quote/market-status",
            Request {
                market: market.into(),
            },
        )
        .await
    }

    /// Get broker holding data for a symbol.
    ///
    /// Path: GET /v1/quote/broker-holding
    pub async fn broker_holding(
        &self,
        symbol: impl Into<String>,
        opts: BrokerHoldingOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/broker-holding",
            Request {
                symbol: symbol.into(),
                period: opts.period,
            },
        )
        .await
    }

    /// Get broker holding detail for a symbol.
    ///
    /// Path: GET /v1/quote/broker-holding/detail
    pub async fn broker_holding_detail(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
        }
        self.http_get_json(
            "/v1/quote/broker-holding/detail",
            Request {
                symbol: symbol.into(),
            },
        )
        .await
    }

    /// Get daily broker holding for a symbol and broker.
    ///
    /// Path: GET /v1/quote/broker-holding/daily
    pub async fn broker_holding_daily(
        &self,
        symbol: impl Into<String>,
        broker_id: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
            broker_id: String,
        }
        self.http_get_json(
            "/v1/quote/broker-holding/daily",
            Request {
                symbol: symbol.into(),
                broker_id: broker_id.into(),
            },
        )
        .await
    }

    /// Get AH premium klines for a symbol.
    ///
    /// Path: GET /v1/quote/ahpremium/klines
    pub async fn ah_premium_klines(
        &self,
        symbol: impl Into<String>,
        opts: AhPremiumKlinesOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            period: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            count: Option<u32>,
        }
        self.http_get_json(
            "/v1/quote/ahpremium/klines",
            Request {
                symbol: symbol.into(),
                period: opts.period,
                count: opts.count,
            },
        )
        .await
    }

    /// Get AH premium timeshares for a symbol.
    ///
    /// Path: GET /v1/quote/ahpremium/timeshares
    pub async fn ah_premium_timeshares(
        &self,
        symbol: impl Into<String>,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
        }
        self.http_get_json(
            "/v1/quote/ahpremium/timeshares",
            Request {
                symbol: symbol.into(),
            },
        )
        .await
    }

    /// Get trade statistics for a symbol.
    ///
    /// Path: GET /v1/quote/trades-statistics
    pub async fn trade_statistics(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
        }
        self.http_get_json(
            "/v1/quote/trades-statistics",
            Request {
                symbol: symbol.into(),
            },
        )
        .await
    }

    /// Get market anomaly data.
    ///
    /// Path: GET /v1/quote/changes
    pub async fn market_anomaly(&self, market: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            market: String,
        }
        self.http_get_json(
            "/v1/quote/changes",
            Request {
                market: market.into(),
            },
        )
        .await
    }

    /// Get index constituents for an index symbol.
    ///
    /// Path: GET /v1/quote/index-constituents
    pub async fn index_constituents(&self, symbol: impl Into<String>) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            symbol: String,
        }
        self.http_get_json(
            "/v1/quote/index-constituents",
            Request {
                symbol: symbol.into(),
            },
        )
        .await
    }

    // -----------------------------------------------------------------------
    // Domain C: Calendar
    // -----------------------------------------------------------------------

    /// Get finance calendar for a market.
    ///
    /// Path: GET /v1/quote/finance_calendar
    pub async fn finance_calendar(
        &self,
        market: impl Into<String>,
        opts: FinanceCalendarOptions,
    ) -> Result<serde_json::Value> {
        #[derive(Serialize)]
        struct Request {
            market: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            end_date: Option<String>,
        }
        self.http_get_json(
            "/v1/quote/finance_calendar",
            Request {
                market: market.into(),
                start_date: opts.start_date,
                end_date: opts.end_date,
            },
        )
        .await
    }
}
