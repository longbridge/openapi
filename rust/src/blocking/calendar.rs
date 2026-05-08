use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    calendar::{CalendarContext, types::*},
};

/// Blocking financial calendar context
pub struct CalendarContextSync {
    rt: BlockingRuntime<CalendarContext>,
}

impl CalendarContextSync {
    /// Create a [`CalendarContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = CalendarContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get financial calendar events
    pub fn finance_calendar(
        &self,
        category: CalendarCategory,
        start: impl Into<String> + Send + 'static,
        end: impl Into<String> + Send + 'static,
        market: Option<String>,
    ) -> Result<CalendarEventsResponse> {
        self.rt.call(
            move |ctx| async move { ctx.finance_calendar(category, start, end, market).await },
        )
    }
}
