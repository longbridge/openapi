use std::{ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::{CalendarContext, calendar::types::*};

use crate::{
    async_call::{CAsyncCallback, execute_async},
    calendar_context::{enum_types::*, types::*},
    config::CConfig,
    types::{CCow, cstr_to_rust},
};

pub struct CCalendarContext {
    ctx: CalendarContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_calendar_context_new(
    config: *const CConfig,
) -> *const CCalendarContext {
    let config = Arc::new((*config).0.clone());
    Arc::into_raw(Arc::new(CCalendarContext {
        ctx: CalendarContext::new(config),
    }))
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_calendar_context_retain(ctx: *const CCalendarContext) {
    Arc::increment_strong_count(ctx);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_calendar_context_release(ctx: *const CCalendarContext) {
    let _ = Arc::from_raw(ctx);
}

/// Get financial calendar events.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_calendar_context_finance_calendar(
    ctx: *const CCalendarContext,
    category: CCalendarCategory,
    start: *const c_char,
    end: *const c_char,
    market: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let cat: CalendarCategory = category.into();
    let start = cstr_to_rust(start);
    let end = cstr_to_rust(end);
    let mkt = if market.is_null() {
        None
    } else {
        Some(cstr_to_rust(market))
    };
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CCalendarEventsResponseOwned> =
            CCow::new(CCalendarEventsResponseOwned::from(
                ctx_inner.finance_calendar(cat, start, end, mkt).await?,
            ));
        Ok(resp)
    });
}
