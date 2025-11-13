# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

- fix [#298](https://github.com/longportapp/openapi/issues/298)

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
- fix(nodejs): correct condition for disabling quote package printing [#230](https://github.com/longportapp/openapi/pull/230)

# [3.0.6] 2025-06-02

- fix: Add missing types register [#226](https://github.com/longportapp/openapi/pull/226)

# [3.0.4] 2025-05-15

- java-sdk: rename `QuoteContext.securityList` to `QuoteContext.getSecurityList`
- java-sdk: add `QuoteContext.getMarketTemperature` and `QuoteContext.getHistoryMarketTemperature` methods

# [3.0.3] 2025-05-14

- fix [#213](https://github.com/longportapp/openapi/issues/213)

# [3.0.1] 2025-05-13

- fix [#212](https://github.com/longportapp/openapi/issues/212)

# [3.0.0] 2025-05-13

- add support extended hours candlesticks
- add market temperature api
- add support use environment variable `LONGPORT_LANGUAGE` to set the response language
- java-sdk: add `QuoteContext.getCapitalDistribution` method
- fix [#208](https://github.com/longportapp/openapi/issues/208)

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

- Add `LONGPORT_PRINT_QUOTE_PACKAGES` environment variable to enable printing the opened quote packages when connected to the server, default is `true`.

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
