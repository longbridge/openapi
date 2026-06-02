use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Serialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{Config, Result, calendar::types::*};

struct InnerCalendarContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerCalendarContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("calendar context dropped");
        });
    }
}

/// Financial calendar context — earnings, dividends, splits, IPOs, macro data.
#[derive(Clone)]
pub struct CalendarContext(Arc<InnerCalendarContext>);

impl CalendarContext {
    /// Create a [`CalendarContext`]
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("calendar");
        dispatcher::with_default(&log_subscriber.clone().into(), || {
            tracing::info!(language = ?config.language, "creating calendar context");
        });
        let ctx = Self(Arc::new(InnerCalendarContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("calendar context created");
        });
        ctx
    }

    /// Returns the log subscriber
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Get financial calendar events.
    ///
    /// The endpoint is paginated via `next_date`. When the returned
    /// `next_date` is non-empty, pass it as `start` to fetch the next page.
    ///
    /// Path: `GET /v1/quote/finance_calendar`
    pub async fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: impl Into<String>,
        end: impl Into<String>,
        market: Option<String>,
    ) -> Result<CalendarEventsResponse> {
        let cat_str = match category {
            CalendarCategory::Report => "report",
            CalendarCategory::Dividend => "dividend",
            CalendarCategory::Split => "split",
            CalendarCategory::Ipo => "ipo",
            CalendarCategory::MacroData => "macrodata",
            CalendarCategory::Closed => "closed",
            CalendarCategory::Meeting => "meeting",
            CalendarCategory::Merge => "merge",
        };
        #[derive(Serialize)]
        struct Query {
            date: String,
            date_end: String,
            #[serde(rename = "types[]")]
            types: &'static str,
            #[serde(rename = "markets[]", skip_serializing_if = "Option::is_none")]
            markets: Option<String>,
        }
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/quote/finance_calendar")
            .query_params(Query {
                date: start.into(),
                date_end: end.into(),
                types: cat_str,
                markets: market,
            })
            .response::<Json<CalendarEventsResponse>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
