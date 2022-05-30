use crate::{trade::types::PushOrderChanged, utils::JsCallback};

mod context;
mod requests;
mod types;

/// Trade context
#[napi_derive::napi]
#[derive(Clone)]
pub struct QuoteContext {
    ctx: longbridge::quote::QuoteContext,
    on_order_changed: JsCallback<PushOrderChanged>,
}
