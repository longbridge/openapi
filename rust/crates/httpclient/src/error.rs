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

/// Returns the recovery hint for a region-restricted API path.
fn region_alternative(path: &str, required: DcRegion) -> Option<&'static str> {
    match (path, required) {
        ("/v1/asset/account", DcRegion::Ap) => {
            Some("use us_cash_and_buy_power or us_asset_overview instead")
        }
        ("/v1/asset/cashflow", DcRegion::Ap) => Some("use us_transaction_history instead"),
        ("/v1/asset/stock", DcRegion::Ap) => Some("use us_asset_overview instead"),
        ("/v1/asset/exchange_rates", DcRegion::Ap) => Some("use us_exchange_rates instead"),
        ("/v1/trade/order", DcRegion::Ap) => Some("use us_order_detail instead"),
        ("/v1/asset/fund", DcRegion::Ap) => Some("there is no US equivalent"),
        ("/v1/risk/margin-ratio", DcRegion::Ap) => {
            Some("there is no US equivalent; see margin_status in us_cash_and_buy_power instead")
        }
        (_, DcRegion::Us) => Some("this API is for US data-center accounts only"),
        _ => None,
    }
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

    #[test]
    fn dc_region_restricted_names_us_alternative() {
        let cases = [
            (
                "/v1/asset/account",
                "use us_cash_and_buy_power or us_asset_overview instead",
            ),
            ("/v1/asset/cashflow", "use us_transaction_history instead"),
            ("/v1/asset/stock", "use us_asset_overview instead"),
            ("/v1/asset/exchange_rates", "use us_exchange_rates instead"),
            ("/v1/trade/order", "use us_order_detail instead"),
            ("/v1/asset/fund", "there is no US equivalent"),
            (
                "/v1/risk/margin-ratio",
                "there is no US equivalent; see margin_status in us_cash_and_buy_power instead",
            ),
        ];
        for (path, hint) in cases {
            assert_eq!(
                restricted(path, DcRegion::Ap),
                format!(
                    "this API ({path}) is only available in the AP data center and is not supported for your US-region account; {hint}"
                ),
                "unexpected message for {path}"
            );
        }
    }

    #[test]
    fn dc_region_restricted_us_only_hint() {
        assert_eq!(
            restricted("/v1/us/assets/overview", DcRegion::Us),
            "this API (/v1/us/assets/overview) is only available in the US data center and is not supported for your AP-region account; this API is for US data-center accounts only"
        );
    }

    #[test]
    fn dc_region_restricted_without_hint_keeps_message() {
        assert_eq!(
            restricted("quote/brokers", DcRegion::Ap),
            "this API (quote/brokers) is only available in the AP data center and is not supported for your US-region account"
        );
    }
}
