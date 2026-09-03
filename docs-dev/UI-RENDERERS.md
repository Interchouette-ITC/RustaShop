# UI clients and renderers

RustaShop exposes **one Commerce API** (Actix + OpenAPI + WebSocket). All UIs are clients. This doc names the **UI plane** RustaShop wants: Angular parity **and** rangular with **dual renderers** (web + native GPU).

## Ambition (rangular direction)

Similar in spirit to GPUix (“React’s model on GPUI”), RustaShop targets:

> **Angular-shaped authoring across Rust web (Leptos/DOM) and native GPU (GPUI).**

That is **more ambitious than a webview desktop shell**: GPUI is a real native renderer, not Chrome embedded in Tauri.

| Track | Authoring                                   | Web renderer            | Native renderer                      |
| ----- | ------------------------------------------- | ----------------------- | ------------------------------------ |
| **A** | Angular (TypeScript)                        | Browser DOM             | (not in v0 scope; Ionic/`app` later) |
| **B** | **rangular** (templates + Rust controllers) | **Leptos** → DOM / wasm | **GPUI** → native GPU                |

Tracks A and B are developed **alike** against the same OpenAPI and realtime contracts: same cart, catalog, checkout flows; different toolchain.

## Layers (do not confuse)

```text
┌─────────────────────────────────────────────────────────────┐
│  UI clients                                                  │
│                                                              │
│  Angular (TS)          rangular (one authoring model)        │
│       │                      │                               │
│       │               ┌──────┴──────┐                        │
│       │               │             │                        │
│       │          Web renderer   Native renderer              │
│       │          Leptos / DOM      GPUI                      │
│       │               │             │                        │
│       └───────────────┴─────────────┘                        │
│                       │ OpenAPI + WS push                    │
└───────────────────────┼──────────────────────────────────────┘
                        ▼
┌─────────────────────────────────────────────────────────────┐
│  Commerce kernel (Serenade app + Actix API + Axum MCP)       │
└─────────────────────────────────────────────────────────────┘
```

**“Web backend Leptos”** and **“desktop backend GPUI”** mean the **UI host / renderer stack** for rangular components, **not** a replacement for the commerce API. Catalog, cart, checkout, and money stay on the Actix kernel.

## Web host: Leptos + rangular

| Mode                               | Role                                                         |
| ---------------------------------- | ------------------------------------------------------------ |
| **CSR wasm** (rangular v0.1 today) | Storefront in browser; Trunk/wasm; demo path exists upstream |
| **SSR / islands** (later)          | Optional Leptos server for SEO, first paint, admin shells    |

RustaShop does not require Leptos full-stack monolith for the kernel. Leptos serves the **web UI host** when track B ships web surfaces.

## Native host: GPUI + rangular

| Item          | Note                                                                        |
| ------------- | --------------------------------------------------------------------------- |
| **Target**    | Desktop admin, operator tools, possibly storefront kiosk                    |
| **Renderer**  | [GPUI](https://github.com/zed-industries/gpui) (GPU-native UI)              |
| **Authoring** | Same rangular templates/controllers where the GPUI backend can compile them |
| **Not**       | Tauri/webview-only desktop (that remains “web path in a window”)            |

GPUI renderer work belongs primarily in **rangular** (new backend target). RustaShop consumes it for admin and native commerce UX.

## Angular track (parallel)

Angular remains **UI option A**: mature ecosystem, separate repo under `clients/angular-*`. Feature parity with rangular track is a **product habit**, not shared code:

- Same API types (OpenAPI codegen)
- Same WS event names
- Same MVP flows (browse → cart → checkout)

## Domains map (reminder)

| Host                    | Typical UI                                     |
| ----------------------- | ---------------------------------------------- |
| `rustashop.io` / `.dev` | Angular or rangular **web** (Leptos)           |
| Desktop installer       | rangular **native** (GPUI)                     |
| `rustashop.app`         | Ionic / mobile (later; likely Angular-aligned) |

## Dependencies and order

1. **Commerce API** MVP on Serenade + Actix (#2, #49, #47)
2. **rangular** web path stable (Leptos CSR) — upstream
3. **rangular GPUI renderer** — new epic upstream (blocks native track)
4. RustaShop **shared commerce components** (templates + services) targeting web first, then native when GPUI backend exists
5. Angular clients in parallel where team capacity allows

## Non-goals (early)

- One binary that is both Actix API and Leptos SSR for everything
- GPUI storefront before admin proves the native renderer
- Forking Angular inside RustaShop

## Related

- [WASM-LAYERS.md](WASM-LAYERS.md) — wasm roles (UI wasm vs plugins vs sandbox)
- [rangular SPEC](https://github.com/Interchouette-ITC/rangular/blob/dev/docs/SPEC.md) — v0.1 browser-only; GPUI is post-v0.1
- GitHub: RustaShop UI renderer epic (see issues)
