# Longbridge OpenAPI SDK

[![](https://img.shields.io/crates/v/longbridge.svg)](https://crates.io/crates/longbridge) [![Go project version](https://badge.fury.io/go/github.com%2Flongbridge%2Fopenapi-go.svg)](https://badge.fury.io/go/github.com%2Flongbridge%2Fopenapi-go) [![PyPI version](https://badge.fury.io/py/longbridge.svg)](https://badge.fury.io/py/longbridge) [![npm version](https://badge.fury.io/js/longbridge.svg)](https://badge.fury.io/js/longbridge) [![Maven Central](https://img.shields.io/maven-central/v/io.github.longbridge/openapi-sdk)](https://search.maven.org/artifact/io.github.longbridge/openapi-sdk)


Longbridge OpenAPI provides programmatic quote trading interfaces for investors with research and development capabilities and assists them to build trading or quote strategy analysis tools based on their own investment strategies. The functions fall into the following categories:

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds
- Real-time subscription - Provides real-time quotes and push notifications for order status changes

**This repo contains the following main components:**

| Name                        | Document                                                              | Description                                       |
|-----------------------------|-----------------------------------------------------------------------|---------------------------------------------------|
| [Rust](rust/README.md)      | [Doc](https://longbridge.github.io/openapi/rust/longbridge/index.html) | Longbridge OpenAPI for Rust `(>= 1.89.0)`           |
| [Python](python/README.md)  | [Doc](https://longbridge.github.io/openapi/python/index.html)        | Longbridge OpenAPI for Python 3 `(>= 3.8)`          |
| [Node.js](nodejs/README.md) | [Doc](https://longbridge.github.io/openapi/nodejs/index.html)        | Longbridge OpenAPI for Node.js `(>= 10)`            |
| [Java](java/README.md)      | [Doc](https://longbridge.github.io/openapi/java/index.html)          | Longbridge OpenAPI for Java `(>= 11)`               |
| [C](c/README.md)            | [Doc](https://longbridge.github.io/openapi/c/index.html)             | Longbridge OpenAPI for C `(>= C99)`                 |
| [C++](cpp/README.md)        | [Doc](https://longbridge.github.io/openapi/cpp/index.html)           | Longbridge OpenAPI for C++`(>= C++17)`              |
| Go                          |                                                                       | https://github.com/longbridge/openapi-go         |
| [MCP](mcp/README.md)        |                                                                       | An MCP server implementation for Longbridge OpenAPI |

## Environment Variables

| Name                           | Description                                                                      |
|--------------------------------|----------------------------------------------------------------------------------|
| LONGBRIDGE_LANGUAGE              | Language identifier, `zh-CN`, `zh-HK` or `en` (Default: `en`)                    |
| LONGBRIDGE_APP_KEY               | App key                                                                          |
| LONGBRIDGE_APP_SECRET            | App secret                                                                       |
| LONGBRIDGE_ACCESS_TOKEN          | Access token                                                                     |
| LONGBRIDGE_HTTP_URL              | HTTP endpoint url (Default: `https://openapi.longbridge.com`)                   |
| LONGBRIDGE_QUOTE_WS_URL          | Quote websocket endpoint url (Default: `wss://openapi-quote.longbridge.com/v2`) |
| LONGBRIDGE_TRADE_WS_URL          | Trade websocket endpoint url (Default: `wss://openapi-trade.longbridge.com/v2`) |
| LONGBRIDGE_ENABLE_OVERNIGHT      | Enable overnight quote, `true` or `false` (Default: `false`)                     |
| LONGBRIDGE_PUSH_CANDLESTICK_MODE | `realtime` or `confirmed` (Default: `realtime`)                                  |
| LONGBRIDGE_PRINT_QUOTE_PACKAGES  | Print quote packages when connected, `true` or `false` (Default: `true`)         |
| LONGBRIDGE_LOG_PATH              | Set the path of the log files (Default: `no logs`)                               |

## Quickstart

- Pick a language SDK and follow its README for install + first request:
  - Rust: `rust/README.md`
  - Python: `python/README.md`
  - Node.js: `nodejs/README.md`
  - Java: `java/README.md`
  - C: `c/README.md`
  - C++: `cpp/README.md`
  - Go: https://github.com/longbridge/openapi-go
- Full reference docs: https://longbridge.github.io/openapi

## SDK Documentation

https://longbridge.github.io/openapi

## Troubleshooting

- Environment variables not taking effect
  - macOS/Linux: `export ...` only affects the current shell session.
  - Windows: `setx ...` requires opening a new terminal/session to take effect.
- Authentication errors (401/403)
  - Verify `LONGBRIDGE_APP_KEY`, `LONGBRIDGE_APP_SECRET`, `LONGBRIDGE_ACCESS_TOKEN` are correct and not expired.
  - Ensure your OpenAPI app has the required permissions.
- Network / connection errors
  - Check firewall/proxy rules for HTTPS/WSS.
  - If you use a custom endpoint, set `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_QUOTE_WS_URL`, `LONGBRIDGE_TRADE_WS_URL`.
- Quote subscription exits immediately
  - Keep the process running (event loop / sleep / blocking receive loop), otherwise you will not see push events.
- Debugging
  - Enable logs via `LONGBRIDGE_LOG_PATH`.
  - If quotes connect but look empty, keep `LONGBRIDGE_PRINT_QUOTE_PACKAGES=true` to confirm opened quote packages.

## Minimal Verification

If you're not sure whether your environment / credentials are correct, start with the built-in HTTP client examples.

- Python:

  ```bash
  python examples/python/http_client.py
  ```

- Node.js:

  ```bash
  node examples/nodejs/http_client.js
  ```

- Rust:

  ```bash
  cargo run --manifest-path examples/rust/Cargo.toml -p http_client
  ```

- Java (from the example module directory):

  ```bash
  cd examples/java/http_client
  mvn -q -DskipTests package
  mvn -q -DskipTests exec:java
  ```

- C/C++:
  - Use the sources in `examples/c/http_client/main.c` and `examples/cpp/http_client/main.cpp`.
  - Build instructions depend on your toolchain; see the corresponding language SDK README.

Expected results:

- If credentials are valid and network is reachable, the HTTP call returns JSON.
- If it returns 401/403, check your `LONGBRIDGE_APP_KEY`, `LONGBRIDGE_APP_SECRET`, `LONGBRIDGE_ACCESS_TOKEN`.
- If it times out / cannot connect, check proxy/firewall and your endpoint env vars.

## Resources

- [Longbridge OpenAPI](https://open.longbridge.com/en/)
- [Longbridge OpenAPI Docs](https://open.longbridge.com/en/docs)

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
