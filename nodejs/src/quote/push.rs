use crate::quote::types::{PushBrokers, PushDepth, PushQuote, PushTrades};

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
            pub fn symbol(&self) -> &str {
                &self.symbol
            }

            #[napi(getter)]
            pub fn data(&self) -> &$ty {
                &self.data
            }
        }
    };
}

define_push_event!(PushQuoteEvent, PushQuote);
define_push_event!(PushDepthEvent, PushDepth);
define_push_event!(PushBrokersEvent, PushBrokers);
define_push_event!(PushTradesEvent, PushTrades);
