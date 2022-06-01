# Longbridge OpenAPI SDK for Rust

<div align="center">
  <a href="https://crates.io/crates/longbridge">
    <img src="https://img.shields.io/crates/v/longbridge.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <a href="https://docs.rs/longbridge">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
      alt="Unsafe Rust forbidden" />
  </a>
  <a href="https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html">
    <img src="https://img.shields.io/badge/rustc-1.56.1+-ab6000.svg"
      alt="rustc 1.56.1+" />
  </a>
</div>


`longbridge` provides an easy-to-use interface for invokes [`Longbridge OpenAPI`](https://open.longbridgeapp.com/en/).

## Quickstart

_Add dependencies to `Cargo.toml`_

```toml
[dependencies]
longbridge = "0.1.0"
```

_Setting environment variables(MacOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables(Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

```rust,no_run
use std::sync::Arc;

use longbridge::{
    decimal,
    trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType},
    Config, QuoteContext, TradeContext,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for quote APIs
    let (ctx, _) = QuoteContext::try_new(config.clone()).await?;

    // Get basic information of securities
    let resp = ctx
        .quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
        .await?;
    println!("{:?}", resp);

    Ok(())
}
```

## Quote API _(Subscribe quotes)_

```rust, no_run
use std::sync::Arc;

use longbridge::{quote::SubFlags, Config, QuoteContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for quote APIs
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;

    // Subscribe
    ctx.subscribe(["700.HK"], SubFlags::QUOTE, true).await?;

    // Receive push events
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }

    Ok(())
}
```

## Trade API _(Submit order)_

```rust, no_run
use std::sync::Arc;

use longbridge::{
    decimal,
    trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType},
    Config, TradeContext,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_env()?);

    // Create a context for trade APIs
    let (ctx, _) = TradeContext::try_new(config).await?;

    // Submit order
    let opts = SubmitOrderOptions::new(
        "700.HK",
        OrderType::LO,
        OrderSide::Buy,
        decimal!(500i32),
        TimeInForceType::Day,
    )
    .submitted_price(decimal!(50i32))
    .remark("Hello from Rust SDK".to_string());

    let resp = ctx.submit_order(opts).await?;
    println!("{:?}", resp);

    Ok(())
}
```

## Crate features

To avoid compiling unused dependencies, Longbridge gates certain features, all of which are disabled by default:

| Feature  | Description                         |
|----------|-------------------------------------|
| blocking | Provides the `blocking` client API. |

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>) at your option.
