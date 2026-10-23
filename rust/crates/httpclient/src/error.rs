use std::error::Error;

use longbridge_geo::DcRegion;
use reqwest::{StatusCode, header::HeaderMap};

use crate::qs::QsError;

/// Http client error type
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    /// Invalid request method
    #[error("invalid request method")]
    InvalidRequestMethod,

    /// Invalid api key
    #[error("invalid api key")]
    InvalidApiKey,

    /// Invalid access token
    #[error("invalid access token")]
    InvalidAccessToken,

    /// Missing environment variable
    #[error("missing environment variable: {name}")]
    MissingEnvVar {
        /// Variable name
        name: String,
    },

    /// Unexpected response
    #[error("unexpected response")]
    UnexpectedResponse,

    /// Request timeout
    #[error("request timeout")]
    RequestTimeout,

    /// The requested API is restricted to one data center and cannot be reached
    /// from a session in a different region.
    ///
    /// The message names the API to call instead when one exists, so callers
    /// (including LLM agents) can recover without parsing the path.
    #[error(
        "this API ({path}) is only available in the {required} data center and is not supported for your {current}-region account{}",
        DcRegionHint::new(.path, *.required)
    )]
    DcRegionRestricted {
        /// The restricted API path (or WebSocket command) that was requested.
        path: String,
        /// The data center this API is limited to.
        required: DcRegion,
        /// The session's current data-center region.
        current: DcRegion,
    },

    /// OpenAPI error
    #[error("openapi error: code={code}: {message}")]
    OpenApi {
        /// Error code
        code: i32,
        /// Error message
        message: String,
        /// Trace id
        trace_id: String,
    },

    /// Deserialize response body
    #[error("deserialize response body error: {0}")]
    DeserializeResponseBody(String),

    /// Serialize request body
    #[error("serialize request body error: {0}")]
    SerializeRequestBody(String),

    /// Serialize query string error
    #[error("serialize query string error: {0}")]
    SerializeQueryString(#[from] QsError),

    /// Bad status
    #[error("status error: {0}")]
    BadStatus(StatusCode),

    /// An HTTP response that could not be parsed as an OpenAPI response.
    #[error("unexpected HTTP response: status={status}, trace_id={trace_id}, body={body}")]
    UnexpectedHttpResponse {
        /// HTTP response status.
        status: StatusCode,
        /// Upstream trace ID, when present.
        trace_id: String,
        /// Original HTTP response headers.
        headers: Box<HeaderMap>,
        /// Original HTTP response body.
        body: String,
    },

    /// Http error
    #[error(transparent)]
    Http(#[from] HttpError),

    /// Connection limit exceeded
    #[error("connections limitation is hit, limit = {limit}, online = {online}")]
    ConnectionLimitExceeded {
        /// The limit of connections
        limit: i32,
        /// The number of online connections
        online: i32,
    },

    /// OAuth error
    #[error("oauth error: {0}")]
    OAuth(String),

    /// Server-sent events stream error
    #[error("sse stream error: {0}")]
    Sse(String),
}

/// Suffix appended to [`HttpClientError::DcRegionRestricted`] messages that
/// tells the caller which API to use instead.
struct DcRegionHint(Option<&'static str>);

impl DcRegionHint {
    fn new(path: &str, required: DcRegion) -> Self {
        Self(region_alternative(path, required))
    }
}

impl std::fmt::Display for DcRegionHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(hint) => write!(f, "; {hint}"),
            None => Ok(()),
        }
    }
}

/// Hint for an AP-only path with no US counterpart.
const NO_US_EQUIVALENT: &str = "there is no US equivalent";

/// Hint for a US-only path with no AP counterpart.
const NO_AP_EQUIVALENT: &str = "there is no AP equivalent";

/// Returns the recovery hint for a region-restricted API path.
///
/// The hint is shared by every binding (Rust, Python, Node.js, Java, C, C++)
/// and by MCP servers built on the SDK, whose method and tool names differ,
/// so it names the alternative by HTTP endpoint rather than by method.
fn region_alternative(path: &str, required: DcRegion) -> Option<&'static str> {
    match required {
        DcRegion::Ap => ap_only_alternative(path),
        DcRegion::Us => us_only_alternative(path),
    }
}

/// Hint for a US-region account calling an AP-only path.
fn ap_only_alternative(path: &str) -> Option<&'static str> {
    let hint = match path {
        "/v1/asset/account" => {
            "use the US endpoint GET /v1/us/assets/cash-and-buy-power or GET \
             /v1/us/assets/overview (TradeContext US methods)"
        }
        "/v1/asset/cashflow" => {
            "use the US endpoint POST /v1/us/assets/transaction-history (TradeContext US methods)"
        }
        "/v1/asset/stock" => {
            "use the US endpoint GET /v1/us/assets/overview (TradeContext US methods)"
        }
        "/v1/asset/exchange_rates" => {
            "the US equivalent is on TradeContext (GET /v1/us/assets/exchange-rates)"
        }
        "/v1/trade/order" => {
            "use the US endpoint GET /v1/us/orders/{order_id} (TradeContext US methods)"
        }
        "/v1/risk/margin-ratio" => {
            "there is no US equivalent; account-level margin is at GET \
             /v1/us/assets/cash-and-buy-power (margin_status)"
        }
        "/v1/asset/fund" | "quote/brokers" | "/v1/quote/operatings" => NO_US_EQUIVALENT,
        _ if path.starts_with("/v1/quote/broker-holding")
            || path.starts_with("/v1/dailycoins/") =>
        {
            NO_US_EQUIVALENT
        }
        _ => return None,
    };
    Some(hint)
}

/// Hint for an AP-region account calling a US-only path.
fn us_only_alternative(path: &str) -> Option<&'static str> {
    let hint = match path {
        "/v1/us/orders/query" => {
            "use the AP endpoint GET /v1/trade/order/today or GET /v1/trade/order/history"
        }
        "/v1/us/assets/overview" => {
            "use the AP endpoints GET /v1/asset/account and GET /v1/asset/stock"
        }
        "/v1/us/assets/transaction-history" => "use the AP endpoint GET /v1/asset/cashflow",
        "/v1/us/assets/exchange-rates" => "use the AP endpoint GET /v1/asset/exchange_rates",
        "/v1/us/assets/cash-and-buy-power" => "use the AP endpoint GET /v1/asset/account",
        "/v1/us/assets/available-holding" => {
            "use the AP endpoint GET /v1/asset/stock (available_quantity)"
        }
        _ if path.starts_with("/v1/us/orders/") => "use the AP endpoint GET /v1/trade/order",
        _ if path.starts_with("/v1/us/") => NO_AP_EQUIVALENT,
        _ => return None,
    };
    Some(hint)
}

/// Represents an HTTP error
#[derive(Debug)]
pub struct HttpError(pub reqwest::Error);

impl From<reqwest::Error> for HttpError {
    #[inline]
    fn from(err: reqwest::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(source) = self.0.source() {
            write!(f, "{}: {}", self.0, source)
        } else {
            self.0.fmt(f)
        }
    }
}

impl std::error::Error for HttpError {}

/// Http client result type
pub type HttpClientResult<T, E = HttpClientError> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    fn restricted(path: &str, required: DcRegion) -> String {
        let current = match required {
            DcRegion::Ap => DcRegion::Us,
            DcRegion::Us => DcRegion::Ap,
        };
        HttpClientError::DcRegionRestricted {
            path: path.to_string(),
            required,
            current,
        }
        .to_string()
    }

    const AP_PREFIX: &str =
        "is only available in the AP data center and is not supported for your US-region account";
    const US_PREFIX: &str =
        "is only available in the US data center and is not supported for your AP-region account";

    #[test]
    fn dc_region_restricted_names_us_alternative() {
        let cases = [
            (
                "/v1/asset/account",
                "use the US endpoint GET /v1/us/assets/cash-and-buy-power or GET /v1/us/assets/overview (TradeContext US methods)",
            ),
            (
                "/v1/asset/cashflow",
                "use the US endpoint POST /v1/us/assets/transaction-history (TradeContext US methods)",
            ),
            (
                "/v1/asset/stock",
                "use the US endpoint GET /v1/us/assets/overview (TradeContext US methods)",
            ),
            (
                "/v1/asset/exchange_rates",
                "the US equivalent is on TradeContext (GET /v1/us/assets/exchange-rates)",
            ),
            (
                "/v1/trade/order",
                "use the US endpoint GET /v1/us/orders/{order_id} (TradeContext US methods)",
            ),
            (
                "/v1/risk/margin-ratio",
                "there is no US equivalent; account-level margin is at GET /v1/us/assets/cash-and-buy-power (margin_status)",
            ),
            ("/v1/asset/fund", "there is no US equivalent"),
            ("quote/brokers", "there is no US equivalent"),
            ("/v1/quote/operatings", "there is no US equivalent"),
            ("/v1/quote/broker-holding", "there is no US equivalent"),
            (
                "/v1/quote/broker-holding/detail",
                "there is no US equivalent",
            ),
            (
                "/v1/quote/broker-holding/daily",
                "there is no US equivalent",
            ),
            ("/v1/dailycoins/query", "there is no US equivalent"),
            ("/v1/dailycoins/create", "there is no US equivalent"),
        ];
        for (path, hint) in cases {
            assert_eq!(
                restricted(path, DcRegion::Ap),
                format!("this API ({path}) {AP_PREFIX}; {hint}"),
                "unexpected message for {path}"
            );
        }
    }

    #[test]
    fn dc_region_restricted_names_ap_alternative() {
        let cases = [
            (
                "/v1/us/orders/123",
                "use the AP endpoint GET /v1/trade/order",
            ),
            (
                "/v1/us/orders/query",
                "use the AP endpoint GET /v1/trade/order/today or GET /v1/trade/order/history",
            ),
            (
                "/v1/us/assets/overview",
                "use the AP endpoints GET /v1/asset/account and GET /v1/asset/stock",
            ),
            (
                "/v1/us/assets/transaction-history",
                "use the AP endpoint GET /v1/asset/cashflow",
            ),
            (
                "/v1/us/assets/exchange-rates",
                "use the AP endpoint GET /v1/asset/exchange_rates",
            ),
            (
                "/v1/us/assets/cash-and-buy-power",
                "use the AP endpoint GET /v1/asset/account",
            ),
            (
                "/v1/us/assets/available-holding",
                "use the AP endpoint GET /v1/asset/stock (available_quantity)",
            ),
            ("/v1/us/assets/pl/realized", "there is no AP equivalent"),
            ("/v1/us/gemini/crypto-overview", "there is no AP equivalent"),
            (
                "/v1/us/stock-info/company-overview",
                "there is no AP equivalent",
            ),
            (
                "/v1/us/quote/financials/statements",
                "there is no AP equivalent",
            ),
        ];
        for (path, hint) in cases {
            assert_eq!(
                restricted(path, DcRegion::Us),
                format!("this API ({path}) {US_PREFIX}; {hint}"),
                "unexpected message for {path}"
            );
        }
    }

    #[test]
    fn dc_region_restricted_without_hint_keeps_message() {
        assert_eq!(
            restricted("/v1/unknown", DcRegion::Ap),
            format!("this API (/v1/unknown) {AP_PREFIX}")
        );
        assert_eq!(
            restricted("/v1/unknown", DcRegion::Us),
            format!("this API (/v1/unknown) {US_PREFIX}")
        );
    }
}
