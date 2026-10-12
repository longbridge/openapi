use std::sync::Arc;

use longbridge_httpcli::{HttpClient, Json, Method};
use serde::Serialize;
use tracing::{Subscriber, dispatcher, instrument::WithSubscriber};

use crate::{
    Config, Result,
    forex::{ForexOrderDetail, ForexQuote, GetForexQuoteOptions, SubmitForexOrderOptions},
};

/// Request body for the forex quote endpoint: required `from`/`to` plus the
/// flattened optional amount fields.
#[derive(Debug, Serialize)]
struct QuoteRequest {
    from: String,
    to: String,
    #[serde(flatten)]
    options: GetForexQuoteOptions,
}

/// Query string for the forex order-query endpoint.
#[derive(Debug, Serialize)]
struct ClientOrderIdQuery {
    client_order_id: String,
}

struct InnerForexContext {
    http_cli: HttpClient,
    log_subscriber: Arc<dyn Subscriber + Send + Sync>,
}

impl Drop for InnerForexContext {
    fn drop(&mut self) {
        dispatcher::with_default(&self.log_subscriber.clone().into(), || {
            tracing::info!("forex context dropped");
        });
    }
}

/// Forex (currency exchange) channel context.
#[derive(Clone)]
pub struct ForexContext(Arc<InnerForexContext>);

impl ForexContext {
    /// Create a [`ForexContext`].
    pub fn new(config: Arc<Config>) -> Self {
        let log_subscriber = config.create_log_subscriber("forex");
        let ctx = Self(Arc::new(InnerForexContext {
            http_cli: config.create_http_client(),
            log_subscriber,
        }));
        dispatcher::with_default(&ctx.0.log_subscriber.clone().into(), || {
            tracing::info!("forex context created");
        });
        ctx
    }

    /// Returns the log subscriber.
    #[inline]
    pub fn log_subscriber(&self) -> Arc<dyn Subscriber + Send + Sync> {
        self.0.log_subscriber.clone()
    }

    /// Get a forex quote.
    ///
    /// `from` / `to` are ISO 4217 currency codes. Provide at most one of
    /// `amount` / `target_amount` in `options`.
    pub async fn quote(
        &self,
        from: impl Into<String>,
        to: impl Into<String>,
        options: impl Into<Option<GetForexQuoteOptions>>,
    ) -> Result<ForexQuote> {
        Ok(self
            .0
            .http_cli
            .request(Method::POST, "/v1/forex/quote")
            .body(Json(QuoteRequest {
                from: from.into(),
                to: to.into(),
                options: options.into().unwrap_or_default(),
            }))
            .response::<Json<ForexQuote>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Submit a forex order.
    ///
    /// A successful return only means the order was **accepted**; conversion is
    /// asynchronous — poll [`ForexContext::order`] with the same
    /// `client_order_id` for the final state.
    ///
    /// Idempotency: reusing a `client_order_id` yields at most one order. On
    /// timeout / 5xx, do not change `client_order_id`: query first, and only
    /// re-quote + resubmit (same `client_order_id`) if no order is found. Each
    /// `quote_id` can be submitted successfully only once (reuse → `603006`).
    pub async fn submit_order(&self, options: SubmitForexOrderOptions) -> Result<()> {
        #[derive(serde::Deserialize)]
        struct Resp {
            #[serde(default)]
            #[allow(dead_code)]
            ok: bool,
        }
        self.0
            .http_cli
            .request(Method::POST, "/v1/forex/order")
            .body(Json(options))
            .response::<Json<Resp>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?;
        Ok(())
    }

    /// Query a forex order by `client_order_id`.
    pub async fn order(&self, client_order_id: impl Into<String>) -> Result<ForexOrderDetail> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/forex/order")
            .query_params(ClientOrderIdQuery {
                client_order_id: client_order_id.into(),
            })
            .response::<Json<ForexOrderDetail>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
