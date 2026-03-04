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
  <a href="https://blog.rust-lang.org/2021/11/01/Rust-1.89.0.html">
    <img src="https://img.shields.io/badge/rustc-1.89.0+-ab6000.svg"
      alt="rustc 1.89.0+" />
  </a>
</div>


`longbridge` provides an easy-to-use interface for invoking [`Longbridge OpenAPI`](https://open.longbridge.com/en/).

## Documentation

- SDK docs: https://longbridge.github.io/openapi/rust/longbridge/index.html
- crates.io: https://crates.io/crates/longbridge
- Longbridge OpenAPI: https://open.longbridge.com/en/

## Examples

Runnable examples live in `examples/rust/`:

- `examples/rust/account_asset/src/main.rs`
- `examples/rust/http_client/src/main.rs`
- `examples/rust/subscribe_quote/src/main.rs`
- `examples/rust/subscribe_candlesticks/src/main.rs`
- `examples/rust/submit_order/src/main.rs`
- `examples/rust/today_orders/src/main.rs`

## Quickstart

_Add dependencies to `Cargo.toml`_

```toml
[dependencies]
longbridge = "4.0.0"
```

### Authentication

Longbridge OpenAPI supports two authentication methods:

#### 1. OAuth 2.0 (Recommended)

OAuth 2.0 uses Bearer tokens without requiring HMAC signatures.  The token is
persisted automatically at `~/.longbridge-openapi/tokens/<client_id>`
(`%USERPROFILE%\.longbridge-openapi\tokens\<client_id>` on Windows) and
refreshed transparently on every request.

**Step 1: Register an OAuth Client**

Register an OAuth client to obtain your `client_id`:

_bash / macOS / Linux_

```bash
curl -X POST https://openapi.longbridgeapp.com/oauth2/register \
  -H "Content-Type: application/json" \
  -d '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

_PowerShell (Windows)_

```powershell
Invoke-RestMethod -Method Post -Uri https://openapi.longbridgeapp.com/oauth2/register `
  -ContentType "application/json" `
  -Body '{
    "client_name": "My Application",
    "redirect_uris": ["http://localhost:60355/callback"],
    "grant_types": ["authorization_code", "refresh_token"]
  }'
```

Response:

```json
{
  "client_id": "your-client-id-here",
  "client_secret": null,
  "client_name": "My Application",
  "redirect_uris": ["http://localhost:60355/callback"]
}
```

**Step 2: Build an `OAuth` handle and create `Config`**

```rust,no_run
use std::sync::Arc;
use longbridge::{Config, oauth::OAuthBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads an existing token from ~/.longbridge-openapi/tokens/<client_id>.
    // If none exists or it is expired, opens the browser authorization flow.
    // Token refresh is handled automatically on every subsequent request.
    let oauth = OAuthBuilder::new("your-client-id")
        // .callback_port(8080)  // optional, default 60355
        .build(|url| println!("Open this URL to authorize: {url}"))
        .await?;

    let config = Arc::new(Config::from_oauth(oauth));

    // Use config to create contexts...
    Ok(())
}
```

**Benefits:**
- No shared secret required
- No per-request signature calculation
- Token lifecycle (load, refresh, persist) managed automatically

#### 2. Legacy API Key (Environment Variables)

For backward compatibility you can use the traditional API key method.

_Setting environment variables (macOS/Linux)_

```bash
export LONGBRIDGE_APP_KEY="App Key get from user center"
export LONGBRIDGE_APP_SECRET="App Secret get from user center"
export LONGBRIDGE_ACCESS_TOKEN="Access Token get from user center"
```

_Setting environment variables (Windows)_

```powershell
setx LONGBRIDGE_APP_KEY "App Key get from user center"
setx LONGBRIDGE_APP_SECRET "App Secret get from user center"
setx LONGBRIDGE_ACCESS_TOKEN "Access Token get from user center"
```

## Quote API _(Get basic information of securities)_

**Using OAuth 2.0 (Recommended):**

```rust,no_run
use std::sync::Arc;
use longbridge::{Config, QuoteContext, oauth::OAuthBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthBuilder::new("your-client-id")
        .build(|url| println!("Open this URL to authorize: {url}"))
        .await?;
    let config = Arc::new(Config::from_oauth(oauth));

    // Create a context for quote APIs
    let (ctx, _) = QuoteContext::try_new(config).await?;

    // Get basic information of securities
    let resp = ctx
        .quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])
        .await?;
    println!("{:?}", resp);

    Ok(())
}
```

**Using legacy API key (environment variables):**

```rust,no_run
use std::sync::Arc;
use longbridge::{Config, QuoteContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_apikey_env()?);

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

```rust,no_run
use std::sync::Arc;
use longbridge::{quote::SubFlags, Config, QuoteContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_apikey_env()?);

    // Create a context for quote APIs
    let (ctx, mut receiver) = QuoteContext::try_new(config).await?;

    // Subscribe
    ctx.subscribe(["700.HK"], SubFlags::QUOTE).await?;

    // Receive push events
    while let Some(event) = receiver.recv().await {
        println!("{:?}", event);
    }

    Ok(())
}
```

## Trade API _(Submit order)_

```rust,no_run
use std::sync::Arc;
use longbridge::{
    decimal,
    trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType},
    Config, TradeContext,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let config = Arc::new(Config::from_apikey_env()?);

    // Create a context for trade APIs
    let (ctx, _) = TradeContext::try_new(config).await?;

    // Submit order
    let opts = SubmitOrderOptions::new(
        "700.HK",
        OrderType::LO,
        OrderSide::Buy,
        decimal!(500),
        TimeInForceType::Day,
    )
    .submitted_price(decimal!(50i32))
    .remark("Hello from Rust SDK".to_string());

    let resp = ctx.submit_order(opts).await?;
    println!("{:?}", resp);

    Ok(())
}
```

## Troubleshooting

- Windows `setx` requires a new terminal; use `set` for the current `cmd.exe` session.
- If you don't see push events, keep the process alive (receiver loop / `sleep`).
- For debugging, set `LONGBRIDGE_LOG_PATH` to enable SDK logs.

## Crate features

To avoid compiling unused dependencies, longbridge gates certain features, all of which are disabled by default:

| Feature  | Description                         |
|----------|-------------------------------------|
| blocking | Provides the `blocking` client API. |

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>) at your option.
