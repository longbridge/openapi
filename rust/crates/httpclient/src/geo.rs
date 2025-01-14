use std::{
    cell::{Cell, RefCell},
    time::{Duration, Instant},
};

// because we may call `is_cn` multi times in a short time, we cache the result
thread_local! {
    static LAST_PING: Cell<Option<Instant>> = const { Cell::new(None) };
    static LAST_PING_REGION: RefCell<String> = const { RefCell::new(String::new()) };
}

fn region() -> Option<String> {
    // check user defined REGION
    if let Ok(region) = std::env::var("LONGPORT_REGION") {
        return Some(region);
    }

    // check network connectivity
    // make sure block_on doesn't block the outer tokio runtime
    let handler = std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(ping())
    });
    handler.join().unwrap()
}

async fn ping() -> Option<String> {
    if let Some(last_ping) = LAST_PING.get() {
        if last_ping.elapsed() < Duration::from_secs(60) {
            return Some(LAST_PING_REGION.with_borrow(Clone::clone));
        }
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
    LAST_PING.set(Some(Instant::now()));
    LAST_PING_REGION.replace(region.to_string());
    Some(region.to_string())
}

/// do the best to guess whether the access point is in China Mainland or not
pub fn is_cn() -> bool {
    region().is_some_and(|region| region.eq_ignore_ascii_case("CN"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var() {
        std::env::set_var("LONGPORT_REGION", "CN");
        assert!(is_cn());

        std::env::set_var("LONGPORT_REGION", "SG");
        assert!(!is_cn());
    }
}
