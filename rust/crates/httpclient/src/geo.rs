use std::{cell::RefCell, time::Duration};

// because we may call `is_cn` multi times in a short time, we cache the result
thread_local! {
    static REGION: RefCell<Option<String>> = const { RefCell::new(None) };
}

async fn region() -> Option<String> {
    // check user defined REGION
    if let Ok(region) = std::env::var("LONGPORT_REGION") {
        return Some(region);
    }

    // check network connectivity
    // make sure block_on doesn't block the outer tokio runtime
    ping().await
}

async fn ping() -> Option<String> {
    if let Some(region) = REGION.with_borrow(Clone::clone) {
        return Some(region.clone());
    }

    let Ok(resp) = reqwest::Client::new()
        .get("https://api.lbkrs.com/_ping")
        .timeout(Duration::from_secs(1))
        .send()
        .await
    else {
        return None;
    };
    let region = resp
        .headers()
        .get("X-Ip-Region")
        .and_then(|v| v.to_str().ok())?;
    REGION.set(Some(region.to_string()));
    Some(region.to_string())
}

/// do the best to guess whether the access point is in China Mainland or not
pub async fn is_cn() -> bool {
    region()
        .await
        .is_some_and(|region| region.eq_ignore_ascii_case("CN"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_var() {
        is_cn().await;

        std::env::set_var("LONGPORT_REGION", "CN");
        assert!(is_cn().await);

        std::env::set_var("LONGPORT_REGION", "SG");
        assert!(!is_cn().await);
    }
}
