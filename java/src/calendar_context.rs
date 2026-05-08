use std::sync::Arc;

use jni::{
    JNIEnv,
    objects::{JClass, JObject},
};
use longbridge::{CalendarContext, Config, calendar::types::*};

use crate::{
    async_util,
    error::jni_result,
    types::{JavaInteger, get_field},
};

struct ContextObj {
    ctx: CalendarContext,
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newCalendarContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: CalendarContext::new(config),
        })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeCalendarContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_calendarContextFinanceCalendar(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let category_val: Option<JavaInteger> = get_field(env, &opts, "category")?;
        let category = match category_val.map(i32::from).unwrap_or(0) {
            1 => CalendarCategory::Dividend,
            2 => CalendarCategory::Split,
            3 => CalendarCategory::Ipo,
            4 => CalendarCategory::MacroData,
            5 => CalendarCategory::Closed,
            _ => CalendarCategory::Report,
        };
        let start: String = get_field(env, &opts, "start")?;
        let end: String = get_field(env, &opts, "end")?;
        let market: Option<String> = get_field(env, &opts, "market")?;
        async_util::execute(env, callback, async move {
            Ok(context
                .ctx
                .finance_calendar(category, start, end, market)
                .await?)
        })?;
        Ok(())
    })
}
