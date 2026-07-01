//! Geo-detection helper for Longbridge OpenAPI.
//!
//! Determines whether the current access point is in China Mainland so that
//! callers can choose between `*.longbridge.cn` and `*.longbridge.com`
//! endpoints.

use std::{
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

// Process-wide cache so the probe is done at most once regardless of which
// tokio worker thread calls `is_cn()`.
static IS_CN_DONE: OnceLock<bool> = OnceLock::new();

// Used to prevent multiple concurrent probes racing at startup.
static IS_CN_PROBING: AtomicBool = AtomicBool::new(false);

/// Do the best to guess whether the access point is in China Mainland or not.
///
/// Detection priority:
/// 1. `LONGBRIDGE_REGION` environment variable (takes precedence).
/// 2. `LONGPORT_REGION` environment variable (fallback alias).
/// 3. Process-wide cached result from a previous probe.
/// 4. Live HTTP probe to `https://geotest.lbkrs.com` — HTTP 200 → CN, anything
///    else (error or non-200) → not CN.
pub async fn is_cn() -> bool {
    // 1 & 2: explicit region override
    let user_region = std::env::var("LONGBRIDGE_REGION")
        .ok()
        .or_else(|| std::env::var("LONGPORT_REGION").ok());
    if let Some(region) = user_region {
        return region.eq_ignore_ascii_case("CN");
    }

    // 3: already probed
    if let Some(&cached) = IS_CN_DONE.get() {
        return cached;
    }

    // 4: live probe — only one task does the actual probe; others fall back
    //    to `false` (global endpoint) which is safe and avoids a pile-up.
    if IS_CN_PROBING
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
    {
        let result = reqwest::Client::new()
            .get("https://geotest.lbkrs.com")
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .is_ok_and(|resp| resp.status().is_success());

        let _ = IS_CN_DONE.set(result);
        result
    } else {
        // Another task is probing; use the cached value if it finished in the
        // meantime, otherwise default to global endpoint.
        IS_CN_DONE.get().copied().unwrap_or(false)
    }
}

/// HTTP and WebSocket header that selects the data center serving a request.
///
/// An absent header is treated as [`DcRegion::Ap`] by the API gateway.
pub const DC_REGION_HEADER: &str = "x-dc-region";

/// Data center region used for API gateway routing.
///
/// Independent of [`is_cn`]: that picks the `*.longbridge.cn` vs
/// `*.longbridge.com` host (mainland acceleration), while this selects which
/// data center (`us`/`ap`) the gateway sources data from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcRegion {
    /// Asia-Pacific data center (`ap`). The gateway default.
    Ap,
    /// US data center (`us`).
    Us,
}

impl DcRegion {
    /// Derive the region from a single credential's prefix.
    ///
    /// Longbridge credentials — the OAuth access token, and the legacy API-key
    /// `app_key` / `app_secret` / `access_token` — are prefixed with their data
    /// center: `us_…` for the US data center, `ap_…` for Asia-Pacific. A `us_`
    /// prefix maps to [`DcRegion::Us`]; everything else — including
    /// `ap_`-prefixed and unprefixed credentials — maps to
    /// [`DcRegion::Ap`], matching the gateway default. A leading `Bearer `
    /// is tolerated so an `Authorization` value can be passed directly.
    pub fn from_credential(credential: &str) -> Self {
        let credential = credential.strip_prefix("Bearer ").unwrap_or(credential);
        if credential.starts_with("us_") {
            DcRegion::Us
        } else {
            DcRegion::Ap
        }
    }

    /// Derive the region from a set of credentials, returning [`DcRegion::Us`]
    /// if any of them carries the `us_` prefix.
    ///
    /// Used for legacy API-key auth, where the `app_key`, `app_secret`, and
    /// `access_token` all carry the region prefix.
    pub fn from_credentials(credentials: &[&str]) -> Self {
        if credentials
            .iter()
            .any(|c| DcRegion::from_credential(c) == DcRegion::Us)
        {
            DcRegion::Us
        } else {
            DcRegion::Ap
        }
    }

    /// The [`DC_REGION_HEADER`] value for this region (`"us"` or `"ap"`).
    pub fn as_str(self) -> &'static str {
        match self {
            DcRegion::Us => "us",
            DcRegion::Ap => "ap",
        }
    }

    /// Strip any leading `Bearer ` from a credential.
    ///
    /// Region prefixes (`hk_m_`, `us_m_`, `ap_m_`, …) are routing metadata
    /// consumed by [`from_credential`] to derive the [`DC_REGION_HEADER`].
    /// The gateway accepts the full prefixed token and routes via the header,
    /// so **no region prefix is stripped** — only `Bearer ` is removed.
    pub fn strip_region_prefix(credential: &str) -> &str {
        credential.strip_prefix("Bearer ").unwrap_or(credential)
    }

    /// HTTP base URL for this region (staging environment).
    pub fn http_url_staging(self) -> &'static str {
        match self {
            DcRegion::Us => "https://openapi-global.longbridge.xyz",
            DcRegion::Ap => "https://openapi.longbridge.xyz",
        }
    }

    /// Quote WebSocket URL for this region (staging environment).
    pub fn quote_ws_url_staging(self) -> &'static str {
        match self {
            DcRegion::Us => "wss://openapi-global-quote.longbridge.xyz",
            DcRegion::Ap => "wss://openapi-quote.longbridge.xyz",
        }
    }

    /// Trade WebSocket URL for this region (staging environment).
    pub fn trade_ws_url_staging(self) -> &'static str {
        match self {
            DcRegion::Us => "wss://openapi-global-trade.longbridge.xyz",
            DcRegion::Ap => "wss://openapi-trade.longbridge.xyz",
        }
    }
}

#[cfg(test)]
mod dc_region_tests {
    use super::*;

    #[test]
    fn from_credential_detects_region() {
        assert_eq!(DcRegion::from_credential("us_abc"), DcRegion::Us);
        assert_eq!(DcRegion::from_credential("ap_abc"), DcRegion::Ap);
        // Unprefixed and unknown prefixes fall back to the AP default.
        assert_eq!(DcRegion::from_credential("abc"), DcRegion::Ap);
        assert_eq!(DcRegion::from_credential(""), DcRegion::Ap);
        // A `Bearer ` prefix is tolerated.
        assert_eq!(DcRegion::from_credential("Bearer us_x"), DcRegion::Us);
        assert_eq!(DcRegion::from_credential("Bearer ap_x"), DcRegion::Ap);
    }

    #[test]
    fn from_credentials_is_us_if_any_is_us() {
        assert_eq!(
            DcRegion::from_credentials(&["ap_key", "us_secret", "ap_token"]),
            DcRegion::Us
        );
        assert_eq!(
            DcRegion::from_credentials(&["ap_key", "ap_secret", "ap_token"]),
            DcRegion::Ap
        );
        assert_eq!(DcRegion::from_credentials(&[]), DcRegion::Ap);
    }

    #[test]
    fn as_str_matches_header_value() {
        assert_eq!(DcRegion::Us.as_str(), "us");
        assert_eq!(DcRegion::Ap.as_str(), "ap");
    }

    #[test]
    fn strip_region_prefix_only_removes_bearer() {
        // Region prefixes are kept as-is; only "Bearer " is stripped.
        assert_eq!(DcRegion::strip_region_prefix("us_m_eyJabc"), "us_m_eyJabc");
        assert_eq!(DcRegion::strip_region_prefix("hk_m_eyJabc"), "hk_m_eyJabc");
        assert_eq!(DcRegion::strip_region_prefix("Bearer us_m_eyJabc"), "us_m_eyJabc");
        assert_eq!(DcRegion::strip_region_prefix("Bearer eyJabc"), "eyJabc");
        assert_eq!(DcRegion::strip_region_prefix("eyJabc"), "eyJabc");
    }
}
