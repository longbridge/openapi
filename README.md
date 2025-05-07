# LongPort OpenAPI SDK

[![](https://img.shields.io/crates/v/longport.svg)](https://crates.io/crates/longport) [![Go project version](https://badge.fury.io/go/github.com%2Flongportapp%2Fopenapi-go.svg)](https://badge.fury.io/go/github.com%2Flongportapp%2Fopenapi-go) [![PyPI version](https://badge.fury.io/py/longport.svg)](https://badge.fury.io/py/longport) [![npm version](https://badge.fury.io/js/longport.svg)](https://badge.fury.io/js/longport) [![Maven Central](https://img.shields.io/maven-central/v/io.github.longportapp/openapi-sdk)](https://search.maven.org/artifact/io.github.longportapp/openapi-sdk)


LongPort OpenAPI provides programmatic quote trading interfaces for investors with research and development capabilities and assists them to build trading or quote strategy analysis tools based on their own investment strategies. The functions fall into the following categories:

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds
- Real-time subscription - Provides real-time quotes and push notifications for order status changes

**This repo contains the following main components:**

| Name                        | Document                                                              | Description                                       |
|-----------------------------|-----------------------------------------------------------------------|---------------------------------------------------|
| [Rust](rust/README.md)      | [Doc](https://longportapp.github.io/openapi/rust/longport/index.html) | LongPort OpenAPI for Rust `(>= 1.56.1)`           |
| [Python](python/README.md)  | [Doc](https://longportapp.github.io/openapi/python/index.html)        | LongPort OpenAPI for Python 3 `(>= 3.7)`          |
| [Node.js](nodejs/README.md) | [Doc](https://longportapp.github.io/openapi/nodejs/index.html)        | LongPort OpenAPI for Node.js `(>= 10)`            |
| [Java](java/README.md)      | [Doc](https://longportapp.github.io/openapi/java/index.html)          | LongPort OpenAPI for Java `(>= 1.8)`              |
| [C](c/README.md)            | [Doc](https://longportapp.github.io/openapi/c/index.html)             | LongPort OpenAPI for C `(>= C99)`                 |
| [C++](cpp/README.md)        | [Doc](https://longportapp.github.io/openapi/cpp/index.html)           | LongPort OpenAPI for C++`(>= C++17)`              |
| Go                          |                                                                       | https://github.com/longportapp/openapi-go         |
| [MCP](mcp/README.md)        |                                                                       | An MCP server implementation for LongPort OpenAPI |

## Environment Variables

| Name                           | Description                                                                      |
|--------------------------------|----------------------------------------------------------------------------------|
| LONGPORT_LANGUAGE              | Language identifier, `zh-CN`, `zh-HK` or `en` (Default: `en`)                    |
| LONGPORT_APP_KEY               | App key                                                                          |
| LONGPORT_APP_SECRET            | App secret                                                                       |
| LONGPORT_ACCESS_TOKEN          | Access token                                                                     |
| LONGPORT_HTTP_URL              | HTTP endpoint url (Default: `https://openapi.longportapp.com`)                   |
| LONGPORT_QUOTE_WS_URL          | Quote websocket endpoint url (Default: `wss://openapi-quote.longportapp.com/v2`) |
| LONGPORT_TRADE_WS_URL          | Trade websocket endpoint url (Default: `wss://openapi-trade.longportapp.com/v2`) |
| LONGPORT_ENABLE_OVERNIGHT      | Enable overnight quote, `true` or `false` (Default: `false`)                     |
| LONGPORT_PUSH_CANDLESTICK_MODE | `realtime` or `confirmed` (Default: `realtime`)                                  |
| LONGPORT_PRINT_QUOTE_PACKAGES  | Print quote packages when connected, `true` or `false` (Default: `true`)         |
| LONGPORT_LOG_PATH              | Set the path of the log files (Default: `no logs`)                               |

## SDK Documenation

https://longportapp.github.io/openapi

## Resources

- [LongPort OpenAPI](https://open.longportapp.com/en/)
- [LongPort OpenAPI Docs](https://open.longportapp.com/en/docs)

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
