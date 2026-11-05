use longbridge_c_macros::CEnum;

/// Transport used by the quote pull APIs
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::QuoteTransport")]
#[allow(clippy::enum_variant_names, non_camel_case_types)]
#[repr(C)]
pub enum CQuoteTransport {
    /// Send pull requests over the quote WebSocket connection (default)
    #[c(remote = "WebSocket")]
    QuoteTransport_WebSocket,
    /// Send pull requests over HTTP (`POST /quote/...`) where the API has a
    /// REST equivalent, falling back to the WebSocket for the rest. Using
    /// only HTTP-backed APIs never opens a WebSocket connection.
    #[c(remote = "Http")]
    QuoteTransport_Http,
}
