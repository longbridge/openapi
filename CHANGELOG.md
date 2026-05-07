# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [Unreleased]

## Added

- **Rust, Go:** ~56 new API methods across `QuoteContext`, `AssetContext`, `TradeContext`, and four new context types.

### QuoteContext — Fundamental Data (counter_id endpoints)
  - `financial_report(symbol, opts)` — GET /v1/quote/financial-reports
  - `institution_ratings(symbol)` — GET /v1/quote/institution-ratings
  - `institution_rating_latest(symbol)` — GET /v1/quote/institution-rating-latest
  - `institution_rating_detail(symbol, opts)` — GET /v1/quote/institution-ratings/detail
  - `dividends(symbol, opts)` — GET /v1/quote/dividends
  - `dividend_detail(symbol, dividend_id)` — GET /v1/quote/dividends/details
  - `forecast_eps(symbol)` — GET /v1/quote/forecast-eps
  - `financial_consensus(symbol)` — GET /v1/quote/financial-consensus-detail
  - `valuation(symbol, opts)` — GET /v1/quote/valuation
  - `valuation_history(symbol, opts)` — GET /v1/quote/valuation/detail
  - `industry_valuation(symbol)` — GET /v1/quote/industry-valuation-comparison
  - `industry_valuation_distribution(symbol)` — GET /v1/quote/industry-valuation-distribution
  - `company_overview(symbol)` — GET /v1/quote/comp-overview
  - `company_executives(symbol)` — GET /v1/quote/company-professionals
  - `shareholders(symbol)` — GET /v1/quote/shareholders
  - `fund_holders(symbol)` — GET /v1/quote/fund-holders
  - `corporate_actions(symbol, opts)` — GET /v1/quote/company-act
  - `investor_relations(symbol)` — GET /v1/quote/invest-relations
  - `operating_data(symbol, opts)` — GET /v1/quote/operatings

### QuoteContext — Market Data (symbol passthrough)
  - `market_status(market)` — GET /v1/quote/market-status
  - `broker_holding(symbol, opts)` — GET /v1/quote/broker-holding
  - `broker_holding_detail(symbol)` — GET /v1/quote/broker-holding/detail
  - `broker_holding_daily(symbol, broker_id)` — GET /v1/quote/broker-holding/daily
  - `ah_premium_klines(symbol, opts)` — GET /v1/quote/ahpremium/klines
  - `ah_premium_timeshares(symbol)` — GET /v1/quote/ahpremium/timeshares
  - `trade_statistics(symbol)` — GET /v1/quote/trades-statistics
  - `market_anomaly(market)` — GET /v1/quote/changes
  - `index_constituents(symbol)` — GET /v1/quote/index-constituents
  - `finance_calendar(market, opts)` — GET /v1/quote/finance_calendar

### AssetContext
  - `exchange_rates()` — GET /v1/asset/exchange_rates

### TradeContext
  - `profit_analysis_summary(opts)` — GET /v1/portfolio/profit-analysis-summary
  - `profit_analysis_sublist(opts)` — GET /v1/portfolio/profit-analysis-sublist
  - `profit_analysis_detail(opts)` — GET /v1/portfolio/profit-analysis/detail

### New module: `alert` (`AlertContext`)
  - `list_alerts()` — GET /v1/notify/reminders
  - `add_alert(opts)` — POST /v1/notify/reminders
  - `delete_alerts(ids)` — DELETE /v1/notify/reminders
  - `enable_alert(id)` — PUT /v1/notify/reminders
  - `disable_alert(id)` — PUT /v1/notify/reminders

### New module: `dca` (`DcaContext`)
  - `list_dca_plans(status?)` — GET /v1/dailycoins/query
  - `create_dca_plan(opts)` — POST /v1/dailycoins/create
  - `update_dca_plan(opts)` — POST /v1/dailycoins/update
  - `pause_dca_plan(plan_id)` — POST /v1/dailycoins/toggle
  - `resume_dca_plan(plan_id)` — POST /v1/dailycoins/toggle
  - `stop_dca_plan(plan_id)` — POST /v1/dailycoins/toggle
  - `dca_history(opts)` — GET /v1/dailycoins/query-records
  - `dca_statistics(symbol?)` — GET /v1/dailycoins/statistic
  - `check_dca_support(opts)` — POST /v1/dailycoins/batch-check-support

### New module: `sharelist` (`SharelistContext`)
  - `list_sharelists(count?)` — GET /v1/sharelists
  - `sharelist_detail(id)` — GET /v1/sharelists/{id}
  - `create_sharelist(opts)` — POST /v1/sharelists
  - `delete_sharelist(id)` — DELETE /v1/sharelists/{id}
  - `add_sharelist_items(id, symbols)` — POST /v1/sharelists/{id}/items
  - `remove_sharelist_items(id, symbols)` — DELETE /v1/sharelists/{id}/items
  - `sort_sharelist_items(id, symbols)` — POST /v1/sharelists/{id}/items/sort
  - `popular_sharelists(count?)` — GET /v1/sharelists/popular

### New module: `quant` (`QuantContext`)
  - `run_quant_script(opts)` — POST /v1/quant/run_script

- **Rust:** `quote::utils::symbol_to_counter_id` helper converts symbols to counter-id format (`ST/MARKET/CODE`).
- **Rust:** New option types in `quote::extra_types` and `trade::extra_types` for the above methods. All new endpoints return `serde_json::Value`.
- **Go:** New option types co-located in each context file. All new endpoints return `json.RawMessage`.

- **Rust:** `Config::header(key, value)` builder method to inject custom headers into every HTTP request and WebSocket upgrade request.
- **Rust, Python:** `ContentContext` adds three new methods:
  - `topic_detail(topic_id)` — get detail of a single topic.
  - `list_topic_replies(opts)` — list replies for a topic, with optional page/size filtering.
  - `create_topic_reply(opts)` — create a reply under a topic.
- **Rust, Python:** New types `ListTopicRepliesOptions`, `CreateReplyOptions`, and `TopicReply` to support the above methods.

## Fixed

- **All bindings:** Correct `SecurityStaticInfo.dividend_yield` doc comment from "Dividend yield" (ratio) to "Dividend" (per share amount) across all language SDKs (Rust, Python, Node.js, Java, C, C++).
- **All bindings:** `create_topic` now returns the topic ID (`String`) instead of `OwnedTopic` to avoid deserialization errors when the API response omits optional fields.

## Changed

- **All bindings:** `ContentContext.topics_mine` renamed to `my_topics`; `ListMyTopicsOptions` renamed to `MyTopicsOptions`.
- **All bindings:** `license` field removed from `OwnedTopic` and `CreateTopicOptions`.

# [4.0.6]

## Added

- **All bindings:** `ContentContext` adds two new methods (Rust, Go, C, C++, Java, Python, Node.js):
  - `my_topics(opts)` — get topics created by the current authenticated user, with optional page/size/topic_type filtering.
  - `create_topic(opts)` — create a new topic; returns the topic ID (`String`) on success.
- **All bindings:** New types `OwnedTopic`, `MyTopicsOptions`, and `CreateTopicOptions` to support the above methods.
- **Python:** Added type stubs (`openapi.pyi`) for `ContentContext`, `AsyncContentContext`, `OwnedTopic`, `TopicReply`, `TopicAuthor`, and `TopicImage`.

## Fixed

- **C++:** `create_topic` callback now correctly yields `std::string` (topic ID) instead of `OwnedTopic`.

# [4.0.5]

## Changed

- **All bindings:** `QuoteContext::new` / `TradeContext::new` / `ContentContext::new` are now synchronous and infallible — no more `await`, `.get()`, or callback at construction time. The WebSocket connection is established lazily on first use.
- **All bindings:** `member_id`, `quote_level`, and `quote_package_details` are now async methods (were previously sync fields/properties).
- **Rust:** A single global Tokio runtime is shared across all SDK components; per-binding runtimes removed.

## Performance

- Reduced connection latency by ~1.3 s by fixing a geo-probe cache issue and a WebSocket rate-limiter initialisation bug.
- Quote: trading days are now loaded lazily on first use instead of eagerly at connect time.

## Fixed

- OAuth token refresh now triggers at 5 minutes before expiry instead of only after expiry, preventing a blocking refresh on the first API call.
- CN region detection updated to use a new probe endpoint.

# [4.0.4]

## Fixed

- **Rust:** Fix copy-paste field mapping bugs in `TryFrom<quote::FilterWarrant> for WarrantInfo` where `strike_price`, `itm_otm`, `implied_volatility`, `delta`, `effective_leverage`, `conversion_ratio`, and `balance_point` were incorrectly mapped to `last_done`. ([#485](https://github.com/longbridge/openapi/pull/485))

# [4.0.3]

## Changed

- Migrate OAuth base URL from `openapi.longbridgeapp.com` to `openapi.longbridge.com`.
- Migrate CN endpoint URLs from `longportapp.cn` to `longbridge.cn`.
- Change OAuth token storage path from `~/.longbridge-openapi/` to `~/.longbridge/openapi/`.
- Update all README docs to use `openapi.longbridge.com` for OAuth registration endpoints.
- Update proto submodule with latest upstream changes (URL migration in proto comments).

# [4.0.2]

## Added

- **All bindings:** New `ContentContext` (Rust, C, C++, Java, Python, Node.js) with two methods:
  - `topics(symbol)` — get discussion topics for a security.
  - `news(symbol)` — get news list for a security.
- **Quote API:** `QuoteContext.filings(symbol)` — get regulatory filings for a security. Available in all bindings (Rust, C, C++, Java, Python, Node.js).
- **MCP server:** Expose `news`, `topics`, and `filings` as MCP tools.

# [4.0.1]

## Fixed

- **Python:** Fix `str()` on enum fields (e.g. `CashFlow.direction`, `Subscription`, `OptionDirection`) causing a hang/deadlock by registering previously missing types in the quote and trade modules. ([#476](https://github.com/longbridge/openapi/issues/476))

# [4.0.0]

## Added

- **OAuth 2.0** authentication for all language bindings (Rust, C, C++, Java, Python, Node.js). Use `OAuthBuilder` to run the browser flow; pass the resulting `OAuth` handle to `Config::from_oauth()`. Tokens are persisted under `~/.longbridge/openapi/tokens/<client_id>` and reused; the browser is only opened when no valid token exists.

- **Python — async callbacks:** `AsyncQuoteContext` and `AsyncTradeContext` accept async callbacks for `set_on_quote`, `set_on_depth`, `set_on_brokers`, `set_on_trades`, `set_on_candlestick`, and `set_on_order_changed`. If a callback returns a coroutine, the SDK schedules it on the asyncio loop. Sync callbacks still work as before.
- **Python — `loop_` parameter:** `AsyncQuoteContext.create()` and `AsyncTradeContext.create()` take an optional `loop_` argument. When using async callbacks, pass `loop_=asyncio.get_running_loop()` so the SDK can schedule coroutines with `asyncio.run_coroutine_threadsafe`. Omit `loop_` when using only sync callbacks.

## Breaking changes

- **Rust:** `Config::new` → `Config::from_apikey`, `Config::from_env` → `Config::from_apikey_env`; removed `Config::refresh_access_token` and `Config::refresh_access_token_blocking`.
- **C/C++:** `lb_config_new` → `lb_config_from_apikey`, `lb_config_from_env` → `lb_config_from_apikey_env`, removed `lb_config_refresh_access_token`; `lb_http_client_new` → `lb_http_client_from_apikey`, `lb_http_client_from_env` → `lb_http_client_from_apikey_env`.
- **Java:** `Config.fromEnv()` → `Config.fromApikeyEnv()`, removed `Config.refreshAccessToken()`.
- **Python:** `Config.from_env()` → `Config.from_apikey_env()`, removed `Config.refresh_access_token()`; `HttpClient.from_env()` → `HttpClient.from_apikey_env()`.
- **Node.js:** `Config.fromEnv()` → `Config.fromApikeyEnv()`.

# [3.0.22]

- python: add asyncio support for quote, trade, and HTTP client; existing sync API unchanged.
- rust: fix incorrect field mapping in `WarrantInfo` for warrant filter API.

# [3.0.21]

- java-sdk: fix `limit_depth_level` and `trigger_count` being correctly passed and read as `Integer` in submit/replace order options and order detail.

# [3.0.20]

- add `limit_depth_level`, `trigger_count`, `monitor_price` to `OrderDetail`, 'Order' types.
- add support specify `limit_depth_level`, `trigger_count`, `monitor_price` when placing order.

# [3.0.18] 2025-11-13

- add `US_VIX` market definition.
- python: add support Python `3.14`.

# [3.0.17] 2025-10-22

- fix candlesticks (K-line) might be generated incorrectly in certain situations.
- fix parsing `OrderDetail` may fail in certain situations.

# [3.0.16] 2025-10-20

- add `SecurityBoard.SPXIndex` and `SecurityBoard.VIXIndex`.

# [3.0.15] 2025-10-13

- add `ErrorKind` enum to represent error kinds.

# [3.0.14] 2025-09-05

- fix candlesticks (K-line) might be generated incorrectly in certain situations.

# [3.0.13] 2025-08-22

- fix [#298](https://github.com/longbridge/openapi/issues/298)

# [3.0.12] 2025-08-08

- add `trade_session` for query all session intraday.
- add `Market.Crypto`.
- fix subscription index K-line.

# [3.0.10] 2025-07-27

- python: fix unable to import SecurityBoard

# [3.0.9] 2025-07-24

- A connection limit exceeded error occurred while creating an OTP.

# [3.0.8] 2025-07-15

- fix: subscribe candlesticks with `Period::Day`.

# [3.0.7] 2025-06-09

- add `AccountBalance.frozen_transaction_fees`
- fix(nodejs): correct condition for disabling quote package printing [#230](https://github.com/longbridge/openapi/pull/230)

# [3.0.6] 2025-06-02

- fix: Add missing types register [#226](https://github.com/longbridge/openapi/pull/226)

# [3.0.4] 2025-05-15

- java-sdk: rename `QuoteContext.securityList` to `QuoteContext.getSecurityList`
- java-sdk: add `QuoteContext.getMarketTemperature` and `QuoteContext.getHistoryMarketTemperature` methods

# [3.0.3] 2025-05-14

- fix [#213](https://github.com/longbridge/openapi/issues/213)

# [3.0.1] 2025-05-13

- fix [#212](https://github.com/longbridge/openapi/issues/212)

# [3.0.0] 2025-05-13

- add support extended hours candlesticks
- add market temperature api
- add support use environment variable `LONGBRIDGE_LANGUAGE` to set the response language
- java-sdk: add `QuoteContext.getCapitalDistribution` method
- fix [#208](https://github.com/longbridge/openapi/issues/208)

# [2.1.8] 2025-01-27

- add `log_path` field to `Config`

# [2.1.6] 2025-01-10

- add support for more candlesticks periods
- add PushQuote.current_volume, PushQuote.current_turnover

# [2.1.5] 2024-12-21

- Add `PushCandlestick.is_confirmed` field.

# [2.1.0] 2024-11-14

- Update candlesticks rule.

# [2.0.5] 2024-11-16

- Add Serialize/Deserialize to response types.

# [2.0.4] 2024-11-15

- Add `LONGBRIDGE_PRINT_QUOTE_PACKAGES` environment variable to enable printing the opened quote packages when connected to the server, default is `true`.

# [2.0.3] 2024-11-14

- Changed the `time` parameter of `Quote.history_candlesticks_by_offset` method to be optional.

# [2.0.2] 2024-10-31

- [python] Change `TradeStatus.SuspendTrade` to `TradeStatus.Suspend` in pyi.

# [2.0.1] 2024-10-22

- Returns the most recent historical candlesticks after subscribing to the candlesticks.

# [2.0.0] 2024-10-09

### Added

- Print the opened quote packages when connected to the server.
- Add `EstimateMaxPurchaseQuantityOptions.fractional_shares` field, sets to `true` to get the maximum fractional share buying power.
- The quantity type in the trading API has changed from `int` to `Decimal`.

# [1.0.32] 2024-08-28

- make Depth.price to optional type
