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
///
/// `count`, `offset` and `next` are optional pagination controls; pass `NULL`
/// to omit any of them and use the server default.
///
/// Events come back grouped by date. `count` caps the events returned for
/// *each date* (server default when omitted), and a group's `count` field is
/// that date's total, so a group holding fewer events than its `count` was
/// truncated: fetch the rest by calling again with `start` = `end` = that
/// date and `offset` set to the number of events already received. The
/// `next_date` cursor points at the next date group and moves past truncated
/// dates. `end` is not a hard bound: the server may return later dates, so
/// filter by date on the client.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_calendar_context_finance_calendar(
    ctx: *const CCalendarContext,
    category: CCalendarCategory,
    start: *const c_char,
    end: *const c_char,
    market: *const c_char,
    count: *const i32,
    offset: *const i32,
    next: *const CCalendarPageDirection,
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
    let count = count.as_ref().copied();
    let offset = offset.as_ref().copied();
    let next = next.as_ref().map(|d| (*d).into());
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CCalendarEventsResponseOwned> =
            CCow::new(CCalendarEventsResponseOwned::from(
                ctx_inner
                    .finance_calendar(cat, start, end, mkt, count, offset, next)
                    .await?,
            ));
        Ok(resp)
    });
}
