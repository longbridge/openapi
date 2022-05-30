# Longbridge OpenAPI SDK

Longbridge OpenAPI provides programmatic quote trading interfaces for investors with research and development capabilities and assists them to build trading or quote strategy analysis tools based on their own investment strategies. The functions fall into the following categories:

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds
- Real-time subscription - Provides real-time quotes and push notifications for order status changes

**This repo contains the following main components:**

| Name                        | Description                                |
|-----------------------------|--------------------------------------------|
| [Rust](rust/README.md)      | Longbridge OpenAPI for Rust `(>= 1.56.1)`  |
| [Python](python/README.md)  | Longbridge OpenAPI for Python 3 `(>= 3.7)` |
| [Node.js](nodejs/README.md) | Longbridge OpenAPI for Npde.js `(>= 10)`   |
| Go                          | WIP                                        |

## How to enable Longbridge OpenAPI

1. Log in to the [Longbridge App](https://longbridgeapp.com) or  the official website [longbridgehk.com](https://longbridgehk.com) to complete the account opening process of Longbridge Integrated A/C (the interface services of the Longbridge Standard A/C are not currently available)

2. Log in to the [longbridgeapp.com](https://longbridgeapp.com) and enter the developer platform, complete the developer verification (OpenAPI permission application), and obtain a token.

## SDK Documenation

https://longbridgeapp.github.io/openapi-sdk

## Pricing

Longbridge does not charge any additional fees for activating or using interface services. You only need to open a Longbridge Integrated A/C and get OpenAPI service permissions to use it for free. Please refer to [Pricing](https://longbridge.hk/rate) or consult online customer service for the actual commissions or advanced quotes fees incurred by transactions.

## Resources

- [Longbridge OpenAPI](https://open.longbridgeapp.com/en/)
- [Longbridge OpenAPI Docs](https://open.longbridgeapp.com/en/docs)

## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT) at your option.
