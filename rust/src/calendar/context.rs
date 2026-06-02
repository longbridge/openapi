use std::sync::Arc;

use std::collections::HashSet;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::{Deserialize, Serialize};
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
    /// The endpoint is paginated via `next_date`; this method follows the
    /// cursor automatically and returns the complete merged result.
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

        // Internal page type that captures the pagination cursor.
        #[derive(Deserialize)]
        struct Page {
            date: String,
            #[serde(default)]
            list: Vec<CalendarDateGroup>,
            #[serde(default)]
            next_date: String,
        }

        let end = end.into();
        let mut cursor = start.into();
        let mut seen: HashSet<String> = HashSet::new();
        let mut first_date = String::new();
        let mut all_groups: Vec<CalendarDateGroup> = Vec::new();

        loop {
            let page = self
                .0
                .http_cli
                .request(Method::GET, "/v1/quote/finance_calendar")
                .query_params(Query {
                    date: cursor,
                    date_end: end.clone(),
                    types: cat_str,
                    markets: market.clone(),
                })
                .response::<Json<Page>>()
                .send()
                .with_subscriber(self.0.log_subscriber.clone())
                .await?
                .0;

            if first_date.is_empty() {
                first_date = page.date;
            }

            for mut grp in page.list {
                grp.infos.retain(|info| seen.insert(info.id.clone()));
                if !grp.infos.is_empty() {
                    grp.count = grp.infos.len() as i32;
                    all_groups.push(grp);
                }
            }

            if page.next_date.is_empty() {
                break;
            }
            cursor = page.next_date;
        }

        Ok(CalendarEventsResponse {
            date: first_date,
            list: all_groups,
        })
    }
}
