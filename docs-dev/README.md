# Developer foundations (`docs-dev`)

Internal orientation for RustaShop before and during early implementation. These notes capture **product and technical identity**: how we want the system to feel and which axes we invest in next to the MVP vertical slice in the [README](../docs/README.md).

Public contributor docs (`docs/ARCHITECTURE.md`, `docs/CONTRIBUTING.md`, …) still belong under the existing docs epics once the Cargo tree exists. **`docs-dev` is the living foundation set** so roadmap debates are not trapped in chat.

## Documents

| Doc                                    | Topic                                                                                    |
| -------------------------------------- | ---------------------------------------------------------------------------------------- |
| [FOUNDATIONS.md](FOUNDATIONS.md)       | Overall identity: contracts, three Wasm roles, realtime default, AI-native, roadmap axes |
| [AI-NATIVE.md](AI-NATIVE.md)           | AI across API and UIs: discovery, agents, catalog, pricing, support, MCP                 |
| [DOMAINS.md](DOMAINS.md)               | Hostnames (`ai` / `io` / `dev` / `app` / redirects) and deploy surfaces                  |
| [UI-RENDERERS.md](UI-RENDERERS.md)     | Angular + rangular dual track; Leptos web vs GPUI native hosts                           |
| [WASM-LAYERS.md](WASM-LAYERS.md)       | Storefront Wasm vs plugin Component Model vs sandbox runtimes                            |
| [REALTIME.md](REALTIME.md)             | WebSocket-first live shop state (Meteor-like opinion, RustaShop protocol)                |
| [EXTENSIONS.md](EXTENSIONS.md)         | WIT plugin ABI, host capabilities, OpenAPI vs WIT                                        |
| [WASMER-SANDBOX.md](WASMER-SANDBOX.md) | Wasmer SDK: polyglot guests, agents, PHP legacy, playgrounds, connectors                 |

## How this relates to the MVP

The README MVP (catalog, cart, checkout, orders, both UIs, OpenAPI, compose) stays the **first proof**. Foundations here say what we **design toward** so early crate and API choices do not paint us into a pure REST monolith with plugins bolted on later.

HTTP: **Actix-web** for the commerce kernel; **Axum** for MCP and agent tool surfaces ([FOUNDATIONS.md](FOUNDATIONS.md)).

## Issues

GitHub epics (created with this foundation set):

| Epic                                                            | Focus                                                    |
| --------------------------------------------------------------- | -------------------------------------------------------- |
| [#47](https://github.com/Interchouette-ITC/RustaShop/issues/47) | HTTP stack: Actix kernel + Axum MCP                      |
| [#31](https://github.com/Interchouette-ITC/RustaShop/issues/31) | Realtime WebSocket-first live state                      |
| [#34](https://github.com/Interchouette-ITC/RustaShop/issues/34) | WIT Component Model extension ABI                        |
| [#37](https://github.com/Interchouette-ITC/RustaShop/issues/37) | Wasmer polyglot sandbox and agent execution              |
| [#43](https://github.com/Interchouette-ITC/RustaShop/issues/43) | AI-native commerce (API + UIs + MCP)                     |
| [#45](https://github.com/Interchouette-ITC/RustaShop/issues/45) | Domains and `:dev` tip (`interchouette.net` / `.ai` / …) |

Child tasks use labels `area:wasm`, `area:realtime`, `area:extensions`, and `area:ai`. Filter the issues list by those labels for the full backlog.
