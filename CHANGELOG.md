# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **Rust SDK:** removed internal `symbol_to_counter_id` / `index_symbol_to_counter_id` conversions from request parameters across `FundamentalContext` (financial\_report, institution\_rating, dividend, forecast\_eps, consensus, valuation, company, shareholder, fund\_holder, corp\_action, invest\_relation, operating, buyback, ratings, business\_segments, etf\_asset\_allocation, and US-series endpoints), `MarketContext` (broker\_holding, broker\_holding\_detail, broker\_holding\_daily, ah\_premium, ah\_premium\_intraday, trade\_stats, constituent), `DCAContext` (list, create, stats, calc\_date), and `AlertContext` (add). These endpoints now send the user-supplied symbol (e.g. `AAPL.US`, `HSI.HK`) directly instead of converting to internal counter-id format (e.g. `ST/US/AAPL`, `IX/HK/HSI`)
- **Rust SDK:** removed remaining `symbol_to_counter_id` conversions from `QuoteContext` (`short_positions`, `option_volume`, `option_volume_daily`, `short_trades`, `us_crypto_overview`), `TradeContext` (`us_query_orders`), `FundamentalContext` (`executive`, `industry_peers`, `valuation_comparison`), `DCAContext` (`check_support`), `SharelistContext` (`add_securities`, `remove_securities`, `sort_securities`), and `PortfolioContext` (`profit_analysis_detail`, `profit_analysis_flows`). All outbound query/body parameters now use the user-supplied symbol string directly
- **Rust SDK:** removed all `counter_id → symbol` response-deserialization conversions. All affected structs (`ExecutiveGroup`, `ShareholderStock`, `FundHolder`, `OperatingFinancial`, `EtfAllocationItem`, `CryptoStaticInfo`, `USOrder`, `USOrderDetail`, `USCryptoEntry`, `USStockEntry`) now read the `symbol` field directly from the server response instead of converting from `counter_id`. The `deserialize_counter_id_as_symbol` helper has been removed from `utils/counter.rs`
### Fixed

- **C++ SDK:** `asset::AssetContext` (`statements` / `statement_download_url`) is now actually built and usable. `longbridge.hpp` has always included `asset_context.hpp`, but `cpp/src/asset_context.cpp` was never listed in `cpp/CMakeLists.txt`, so the class was declared to users and then failed to link. It had also never compiled: it included neither `longbridge.h` nor the C declarations, and `statement_download_url` read `res->data` as a `lb_statement_download_url_response_t*` — a type that does not exist anywhere in the C layer, which delivers the URL as a bare `const char*` (the same convention as `QuoteContext::quote_level`). Fixed the include and the callback, and added the file to the build
- **C SDK:** export `lb_statement_item_t` from `longbridge.h`. `CStatementItem` is only reachable through the `void*` async-result pointer, so cbindgen did not emit it and no C or C++ caller could read what `lb_asset_context_statements` returns. Also added the missing `CAssetContext` → `lb_asset_context_t` entry to the cbindgen rename map: every other context type was mapped, so the header exposed the raw Rust name (`const struct CAssetContext *lb_asset_context_new(...)`) while the C++ side forward-declared `lb_asset_context_t`
- **C/C++ SDKs:** every list argument that crosses the FFI boundary now tolerates a null pointer with a zero length. `std::vector::data()` is allowed to return `nullptr` for an empty vector, which is exactly what the C++ binding passes for an omitted list argument, but the C layer fed it straight to `std::slice::from_raw_parts` — undefined behaviour that **aborts the process** under the debug UB checks. Hit live by `QuoteContext::warrant_list` with no filters (`c/src/quote_context/context.rs`); all 17 call sites across `quote_context`, `trade_context`, `agent_context`, `alert_context`, and `types` now go through a null-tolerant `slice_from_raw_parts` helper

### Added

- **Rust:** `Signal.status` is now a `SignalStatus` enum (pending / active / deleted / ai-failed / filtered-by-manual / ai-submit-failed), `SignalsResponse.total` is `i32` to match the wire contract, and the `risk_level` / `display_control` fields were dropped — neither is part of the API contract nor served in production
- **Rust:** `SignalContext.security_facts` now returns a typed `SecurityFact` instead of raw JSON — fact id / type / direction, the securities it is about, the factors behind it (with their anomaly test and groups), the data sources, and the natural-language `nl_info`. Adds the `FactType` and `FactDirection` enums, and `FactNlInfo::summary_tags()` / `invest_anal_tags()` / `eli_explain_tags()` for the `{tag, value}` entries the API carries as JSON inside a string
- **Rust:** add `SignalContext` — strategy signals and the catalyst facts behind them. `signals` (`GET /v1/signals`) queries signals with symbol / strategy / catalyst / time-range filters and paging; `signal` (`GET /v1/signals/{signal_id}`) returns one signal including the full strategy analysis in `json_data`; `security_facts` (`GET /v1/facts/security_facts`) lists a security's fact (catalyst) events. Bindings for the other languages are not wired up yet
- **All languages:** add `TradeContext.submit_multileg` (`POST /v1/trade/order/multileg`) — submit a multi-leg option combination order (vertical spreads, straddles, strangles, collars, etc.) whose legs are placed together as a single strategy order. Takes `side`, `order_type`, `submitted_quantity`, `strategy` (`MultiLegStrategy`), a list of legs (`symbol` + `ratio_quantity`), and optional `submitted_price` / `remark` / `client_request_id`; returns the existing `SubmitOrderResponse`
- **All languages:** order queries and the order push now expose multi-leg strategy information. `Order` (from `today_orders` / `history_orders`), `OrderDetail` (from `order_detail`), and the `PushOrderChanged` order-changed event gain an optional `multi_leg` field (`MultiLegInfo`) — present only for multi-leg option combination orders — carrying the `strategy`, `strategy_name`, `multileg_id`, `code`, and the combination `legs` (each with `symbol`, `side`, `position`, `ratio_quantity`, `strike_price`, `expire_date`, and `contract_direction`). Adds the `MultiLegStrategy`, `MultiLegPosition`, and `ContractDirection` enums
- **All languages:** add grid-trading support via a standalone `GridContext` — submit / replace / cancel / suspend / restart grid orders, list orders (paged and by IDs), fetch order detail and trigger history, submit the strategy risk-disclosure questionnaire, and query the security (symbol) info (`symbol_info` → `GridSymbolInfo`: name, last price, lot sizes, price-step rules, channel/authorization) needed to build a grid order. Available in the Rust, Python, Node.js, Java, and C/C++ bindings
- **All languages:** `Execution` gains a `side` field (`OrderSide`) — the buy/sell direction of the fill, now returned by the `today_executions`, `history_executions`, and `all_executions` responses

### Fixed

- **All languages:** the AI Agent streamed conversation no longer errors mid-run when the server sends an explicit `"outputs": null`. `WorkflowFinishedPayload.outputs`, `NodeToolUseFinishedPayload.outputs`, and `SubagentFinishedPayload.outputs` were annotated `#[serde(default)]`, which only covers a *missing* key, not an explicit `null` — so a `workflow_finished` / `node_tool_use_finished` / `subagent_finished` event carrying `null` outputs failed to deserialize and aborted the whole event stream (`invalid type: null, expected struct WorkflowOutputs`). These fields now map `null` to the type's default
- **All languages:** likewise, list-typed fields on the streamed AI Agent event payloads no longer error on an explicit `null` (`invalid type: null, expected a sequence`). `tip_chips` (on the node / subagent / agent-tool `*_started` payloads), `WorkflowFinishedPayload.process_data`, and `SubagentStartedPayload.tools` were `#[serde(default)]`, which does not accept an explicit `null`; they now deserialize `null` to an empty list
- **All languages:** clarified that `CompanyOverview.employees` is typed as a **string** (not an integer) across all SDK languages — the Longbridge API returns this field as a JSON string (e.g. `"10000"`). Doc comments have been updated to make this explicit and prevent downstream tools from incorrectly treating the value as an integer

## [4.5.0] - 2026-08-14

### Added

- **All languages:** add `AgentContext.public_agents` (`GET /v1/ai/agents`) — list all publicly available Agents on the platform (the Explore catalog). Unlike `agents`, it is not scoped to a Workspace and returns every published, publicly-shared Agent. Takes the same optional `page` / `limit` / `name` parameters and returns the existing `AgentsResponse`
- **All languages:** add optional `parent_message_id` parameter to the AI Agent `conversation` and `conversation_streamed` methods — pass the `message_id` from a previous response to attach a follow-up message after the specified one, keeping the message stream in order. Only valid together with `chat_uid`; must not be set for a new conversation
- **All languages:** the AI Agent SDK now surfaces several response fields it previously dropped or modeled incompletely (Rust, Python, Node.js, Java, and C/C++ bindings):
  - `ConversationResponse.further_questions` — the "you might also ask" follow-up suggestions carried in the `workflow_finished` event's `outputs`
  - `Reference` now captures the full source payload the server sends — `original_index`, `ref_type` (wire `type`), `id`, and the nested `content` (raw JSON; JSON string in C/C++/Java). Previously only a flat `{index, title, url}` was modeled, so `title`/`url` came back empty for real references and `source`/`description`/`published_at`/… were lost entirely. Applies wherever references appear (`ConversationResponse.references`, `message` / `node_tool_use_finished` / `workflow_finished` outputs)
  - `ChatStartedPayload` now carries `chat_id`, `error`, and `error_message` (present on the wire, mirroring `ChatFinishedPayload`)
  - `Interrupt` now exposes `interactions` — a list of `HumanInteraction` (tool call id, interrupt id, interaction type, tool name, questions, and the raw tool arguments) — and `QuestionOption` now exposes `label`. A `null` `questions` / `interactions` list is accepted and deserialized as an empty list instead of erroring

### Changed

- **All languages:** every `HttpClient` now shares a single process-wide `reqwest::Client` (connection pool, DNS cache and TLS state) instead of creating its own. Each SDK context previously built two independent connection pools, so a process that churns thousands of short-lived contexts spun up thousands of pools; they now all share one. `reqwest::Client` is internally reference-counted, all requests target the same OpenAPI host, and auth is applied per-request, so sharing is both correct and far cheaper

### Fixed

- **All languages:** the background reconnect loop in `TradeContext` / `QuoteContext` now stops when the context is dropped. Previously the reconnect loop never observed the shutdown signal, so a context dropped (e.g. evicted from a connection cache) while its server was unreachable would keep reconnecting forever, leaking the task, its HTTP client and connection pool, and the WebSocket state. Under long-running, high-churn workloads (many short-lived contexts) these zombie tasks accumulated and slowly exhausted memory. The reconnect loop now races each attempt against a shutdown channel that closes as soon as the owning context is dropped, so teardown is immediate even mid-reconnect
- **Java SDK:** invalid arguments passed across the JNI boundary no longer crash the JVM (process abort / core dump). Previously a Rust `panic!`/`unwrap`/`expect` in the value conversions unwound across the `extern "system"` boundary and aborted the whole process. These cases now throw a catchable `java.lang.IllegalArgumentException` instead:
  - a `null` or unrecognized constant passed for any enum argument (e.g. `Language`, `OrderSide`, `OrderType`, `Period`, `Market`, `AdjustType`, …)
  - a `null` `BigDecimal` / `LocalDate` / `LocalTime` / `OffsetDateTime` / `String` argument, or an out-of-range date/time, an out-of-range/non-representable decimal, or a non-UTF-8 string
  - `jni_result` now preserves an already-pending Java exception instead of throwing a second one
- **Java SDK:** background callback threads (async request completions and quote/trade push events) no longer abort the JVM on failure. Previously a failed `unwrap` while attaching the tokio worker thread to the JVM, or while converting a result/push payload to a Java object, would panic and crash the whole process; these paths now fail gracefully (the affected callback/event is skipped or delivered as an error). `JniError::throw` no longer calls `FatalError` (which aborts the JVM) as a fallback, and `into_error_object` no longer panics if the error object cannot be built
- **Java SDK:** native-handle-backed objects (`Config`, `HttpClient`, `OAuth`, and every `*Context`, including `AgentContext`) are now memory-safe under concurrent use and close. Previously `close()` freed the handle unconditionally and instance methods dereferenced it without synchronization, so a second/concurrent `close()` (double `Box::from_raw` — a double-free) or a method call racing `close()` (use-after-free) could corrupt the heap and crash the JVM (native `SIGSEGV`) — a race easily hit when handles are pooled and evicted on a background thread (e.g. a Caffeine cache) while worker threads still use them. Now `close()` is idempotent and, together with every instance method that passes a handle across the JNI boundary, `synchronized`, so the two are mutually exclusive; async methods clone the (`Arc`-backed) handle before spawning so an in-flight request keeps it alive, and the cross-object factories (`TradeContext.create(config)`, `Config.fromOAuth(oauth)`, `HttpClient.fromOAuth(oauth)`, …) hold the source handle's monitor while reading it. Any call made after — or losing the race with — `close()` now throws a catchable `IllegalStateException` instead of dereferencing freed memory

## [4.4.3] - 2026-07-30

### Fixed

- **All languages:** fix `option_volume` and `option_volume_daily` response deserialization — actual API fields (`c`/`p` for real-time; `underlying_counter_id`, `total_*` for daily) now correctly mapped; `underlying_counter_id` converted to user-facing symbol; `OptionVolumeStats` simplified to `symbol`, `call_volume`, `put_volume`; `OptionVolumeDailyStat` gains `symbol`, `total_volume`, `total_open_interest` and removes fields not returned by the API

### Changed

- **Node.js SDK:** remove musl (x86_64-unknown-linux-musl) target support

## [4.4.2] - 2026-07-30

### Added

- **All languages:** corrected `OptionVolumeStats` and `OptionVolumeDailyStat` to match the actual API response — fields now are `symbol`, `call_volume`, `put_volume`, `call_open_interest`, `put_open_interest`, `pc_vol` (f32/float), `pc_oi` (f32/float); `OptionVolumeDaily` gains a top-level `symbol` field; `OptionVolumeDailyStat` date field renamed from `timestamp` to `date` (YYYY-MM-DD string)
- **Python SDK:** added missing `option_volume` and `option_volume_daily` stub definitions to `openapi.pyi` — `QuoteContext` and `AsyncQuoteContext` now expose correct signatures and docstrings for these methods
- **All languages:** attached order (take-profit / stop-loss) support for `submit_order` and `replace_order`
  - New types: `AttachedOrderType` (`ProfitTaker` / `StopLoss` / `Bracket`), `AttachedOrderDetail`, `SubmitAttachedParams`, `ReplaceAttachedParams`
  - `SubmitOrderOptions` / `ReplaceOrderOptions`: new `attached_params` field
  - `GetTodayOrdersOptions`: new `is_attached` flag — when combined with `order_id`, treats `order_id` as an attached sub-order ID for lookup (has no effect without `order_id`)
  - `Order` / `OrderDetail`: new `attached_orders: Vec<AttachedOrderDetail>` field
  - New method `order_detail_attached(order_id)` — queries detail for an attached order by its own ID
  - `order_detail` now accepts `GetOrderDetailOptions` (with optional `is_attached` flag) in addition to a plain order ID string
  - `cancel_order` gains an `is_attached` flag to cancel an attached sub-order by its own order ID (Rust: `CancelOrderOptions`; Python: `is_attached` keyword arg; Node.js: optional `isAttached` param; C++/Java: `is_attached` overload/default parameter; C: new `lb_trade_context_cancel_order_attached`)

### Breaking changes

- **All languages:** `OrderDetail.charge_detail` is now `Option<OrderChargeDetail>` (previously non-optional). Attached orders return `null` for this field; callers must handle the absent case.
- **C SDK:** `lb_order_detail_t` gains a new `has_charge_detail: bool` field before `charge_detail`. Existing binaries must be recompiled; code that reads `charge_detail` directly should check `has_charge_detail` first.

## [4.4.1] - 2026-07-22

### Added

- **All languages:** paper trading mode support via `Config`.

### Changed

- **All languages:** `all_executions` temporarily disabled pending API availability.

### Fixed

- **Rust:** preserve the status, trace ID, headers, and raw body of non-OpenAPI HTTP error responses instead of reducing them to a status code.

## [4.4.0] - 2026-07-20

### Added

- **All languages:** **US market APIs** — 14 new interfaces for US-region accounts (requires `us_` token):
  - Fundamental: `us_company_overview`, `us_valuation_overview`, `us_financial_overview`, `us_financial_statement`, `us_key_financial_metrics`, `us_analyst_consensus`, `us_etf_dividend_info`, `us_company_dividends`, `us_etf_files`
  - Quote: `us_crypto_overview` (e.g. `BTCUSD.BKKT`)
  - Trade: `us_asset_overview`, `us_realized_pl`, `us_query_orders`, `us_order_detail`
- **All languages:** **DC-region routing** — `x-dc-region` header auto-derived from token prefix (`us_` → US, others → AP)
- **All languages:** `submit_order` gains optional `client_request_id` for idempotency control.
- **All languages:** new `all_executions` (`GET /v3/trade/execution/all`) with pagination.
- **All languages:** `OutsideRTH` enum gains `OptionPreMarket` for overnight option orders.

### Changed

- **All languages:** `OrderTag` enum: `GTC` renamed to `Gtc`; undocumented variants removed.

## [4.3.3] - 2026-06-26

### Added

- **Rust:** `market::TradeStatus` models `/v1/quote/market-status` trade status codes, including engine-compatible normalization and display helpers.

### Fixed

- **All languages:** corrected market trade status documentation and aligned `market::TradeStatus` with the status definition table, including code `2001` and the `123`/`1009`/`1010` display names.
- **All languages:** `macroeconomic` detail endpoint now populates `info.periodicity` and `info.importance` (`frequence`/`importance` fields added to `V2MacroIndicatorDetail`).

## [4.3.2] - 2026-06-13

### Added

- **All languages:** `macroeconomic_indicators` gains `keyword` parameter for fuzzy name filtering
- **All languages:** `macroeconomic` switches to `GET /v2/quote/macrodata/{id}`, defaults to `sort=desc`

### Changed

- `MacroeconomicIndicator.name` / `.describe`: `MultiLanguageText` → `string`
- `Macroeconomic.unit` / `.unit_prefix`: `MultiLanguageText` → `string`

## [4.3.1] - 2026-06-12

### Added

- **All languages:** `FundamentalContext` gains `macroeconomic_indicators(country, offset, limit)` — list macroeconomic indicators via `GET /v1/quote/macrodata`; filter by country (`MacroeconomicCountry::HongKong / China / UnitedStates / EuroZone / Japan / Singapore`); response includes `count` (total matching)
- **All languages:** `FundamentalContext` gains `macroeconomic(indicator_code, start_date, end_date, offset, limit)` — historical data for a specific indicator via `GET /v1/quote/macrodata/{indicator_code}`; `start_date` / `end_date` accept `"YYYY-MM-DD"` strings; response includes `count` (total data points)
- New types: `MultiLanguageText`, `MacroeconomicCountry`, `MacroeconomicImportance`, `MacroeconomicIndicator`, `MacroeconomicIndicatorListResponse`, `Macroeconomic`, `MacroeconomicResponse`

### Fixed

- `MacroeconomicIndicator.describe` / `name` / `MacroeconomicResponse.info`: handle `null` responses from API without deserializing error

## [4.3.0]

### Added

- **All languages:** `FundamentalContext` gains `etf_asset_allocation(symbol)` — queries `GET /v1/quote/etf-asset-allocation` for ETF asset allocation grouped by element type (`Holdings` / `Regional` / `AssetClass` / `Industry`); returns `AssetAllocationResponse` with report date, position ratios, localized names, and per-holding detail
- **Rust:** new public `longbridge::counter` module — `symbol_to_counter_id`, `index_symbol_to_counter_id`, `counter_id_to_symbol`, and `is_etf`, backed by the embedded ETF + index + warrant directory, so downstream consumers (CLI / MCP) no longer need their own copies
- **Rust:** `QuoteContext` gains `symbol_to_counter_ids(symbols)` (batch conversion via `POST /v1/quote/symbol-to-counter-ids`) and `resolve_counter_ids(symbols)` (local-first resolution with remote fallback) — remotely resolved entries are persisted to `~/.longbridge/cache/counter-ids.csv` (one counter_id per line, override the directory with `LONGBRIDGE_CACHE_DIR`) and consulted by subsequent `counter` lookups, so symbols missing from the embedded directory (e.g. newly listed ETFs) resolve correctly after the first query

### Changed

- `symbol_to_counter_id` now also consults the embedded index and warrant directories — e.g. `HSI.HK` → `IX/HK/HSI`, `10005.HK` → `WT/HK/10005`; leading zeros are stripped from numeric `.HK` codes (`00700.HK` → `ST/HK/700`, A-share codes are kept verbatim)

### Fixed

- Refreshed the embedded US ETF list (4574 → 7250 entries, from the instrument-management export) and added index (648) + warrant (17693) directories — newer ETFs (e.g. `DRAM.US`) were resolved to `ST/...` instead of `ETF/...` counter IDs, breaking ETF-specific APIs such as `etf_asset_allocation`

## [4.2.2]

### Fixed

- **All languages:** `CalendarEventsResponse` now exposes `next_date` cursor — callers can pass it as `start` (with the same `end`) to fetch the next page of `/v1/quote/finance_calendar` results
- **All languages:** `CalendarEventInfo.symbol` now returns standard symbol format (e.g. `CRM.US`) instead of raw `counter_id` format (e.g. `ST/US/CRM`)

## [4.2.1]

### Changed

- `ScreenerContext`: screener endpoints migrated to `/v1/quote/ai/screener/*`; `screener_recommend_strategies` / `screener_user_strategies` now accept a `market` parameter; `screener_search` accepts typed `ScreenerCondition` objects (Mode B) instead of raw strings

### Fixed

- `OperatingFinancial`: renamed `counter_id` → `symbol` (converts `ST/US/AAPL` → `AAPL.US`)

## [4.2.0]

### Added

- 19 new APIs: `FundamentalContext` +9, `QuoteContext` +1 (`short_trades`), `MarketContext` +3, new `ScreenerContext` +5 — see PR [#526](https://github.com/longbridge/openapi/pull/526), [#527](https://github.com/longbridge/openapi/pull/527)
- **Rust:** `OAuthBuilder` gains `TokenStorage` trait for custom token persistence

### Changed

- `short_positions` unified for HK+US; typed structs with RFC 3339 timestamps
- `top_movers`, `rank_list`, `valuation_comparison`: typed structs, `counter_id` → symbol, RFC 3339 timestamps

### Breaking changes

- `stock_events` → `top_movers`; `StockEventsResponse` → `TopMoversResponse`
- `hk_short_positions` removed; use `short_positions(symbol, count)`
- `ShortPositionsResponse`, `ShortTradesResponse`, `TopMoversResponse`, `RankListResponse`, `ValuationComparisonResponse` changed from raw JSON to typed structs

# [4.1.0]

## Breaking changes

- **All languages (Rust, Python, Node.js, Java, C, C++):** `AlertContext::enable()` and `AlertContext::disable()` have been replaced by a single `AlertContext::update(item, enabled)` method. Pass the `AlertItem` from `list()` directly — `enabled = true` enables, `enabled = false` disables. This fixes `invalid frequency` / `invalid indicator id` API errors caused by the old methods sending incomplete fields.

# [4.0.6]

## Added

- **All languages (Rust, Python, Node.js, Java, C, C++):** Seven new context types covering all major data APIs:
  - `FundamentalContext` — financial reports, analyst ratings, dividends, EPS forecasts, consensus estimates, valuation (PE/PB/PS), industry valuation, company overview, executives, shareholders, fund holders, corporate actions, investor relations, operating reports, buyback data, stock ratings.
  - `MarketContext` — market status, broker holding (top/detail/daily), A/H premium (klines/intraday), trade statistics, market anomalies, index constituents.
  - `CalendarContext` — finance calendar (earnings, dividends, splits, IPOs, macro data, market closures, meetings, mergers).
  - `PortfolioContext` — exchange rates, P&L analysis (summary/detail/by-market/flows).
  - `AlertContext` — price alert management (list/add/delete/enable/disable).
  - `DCAContext` — dollar-cost-averaging plan management (list/create/update/pause/resume/stop/history/stats/check-support/calc-date/set-reminder).
  - `SharelistContext` — community sharelist management (list/detail/popular/create/delete/add-securities/remove-securities/sort-securities).
- **All languages:** `QuoteContext` gains `short_positions`, `option_volume`, `option_volume_daily`, and `update_pinned`.
- **All languages:** `ContentContext` gains `topic_detail`, `list_topic_replies`, and `create_topic_reply`.
- **Rust:** `Config::header(key, value)` builder method for injecting custom HTTP/WebSocket headers.
- **All languages (Rust, Python, Node.js, Java, C, C++):** Restore `Config::refresh_access_token` (and `refresh_access_token_blocking` in Rust). Refreshes the access token via the Longbridge token-refresh API. Only available with **Legacy API Key** authentication (`Config::from_apikey`); not supported in OAuth 2.0 mode.

## Changed

- **All languages:** Method parameters now use typed enums instead of raw integers: `DCAFrequency`, `DCAStatus`, `AlertCondition`, `AlertFrequency`, `CalendarCategory`, `FinancialReportKind`, `FinancialReportPeriod`, `BrokerHoldingPeriod`, `AhPremiumPeriod`.
- **All languages:** Response struct fields are typed enums where applicable: `DcaPlan.status` / `invest_frequency` / `market`, `MarketTimeItem.market`, `FlowItem.direction`, `ProfitSummaryInfo.asset_type`, `InstitutionRatingSummary.recommend`.
- **All languages:** All SDK responses are fully typed structs — no method returns a raw JSON string.
- **All languages:** Monetary/numeric fields use `Decimal`/`Option<Decimal>` (Rust) or `BigDecimal` (Java). Non-parseable values such as `""` or `"--"` deserialize as `None`/`null`.

## Fixed

- **Rust:** Fix incorrect cache expiry checks in `QuoteContext`.

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
