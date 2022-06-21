use crate::quote::types::{PushBrokers, PushCandlestick, PushDepth, PushQuote, PushTrades};

macro_rules! define_push_event {
    ($name:ident, $ty:ty) => {
        #[napi_derive::napi]
        #[derive(Debug)]
        pub struct $name {
            pub(crate) symbol: String,
            pub(crate) data: $ty,
        }

        #[napi_derive::napi]
        impl $name {
            #[napi(getter)]
            #[inline]
            pub fn symbol(&self) -> &str {
                &self.symbol
            }

            #[napi(getter)]
            #[inline]
            pub fn data(&self) -> $ty {
                self.data.clone()
            }

            #[napi]
            pub fn to_string(&self) -> String {
                ::std::format!("{:?}", self)
            }
        }
    };
}

define_push_event!(PushQuoteEvent, PushQuote);
define_push_event!(PushDepthEvent, PushDepth);
define_push_event!(PushBrokersEvent, PushBrokers);
define_push_event!(PushTradesEvent, PushTrades);
define_push_event!(PushCandlestickEvent, PushCandlestick);
