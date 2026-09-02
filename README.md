# **rustashop**

**One Rust commerce API. Two UIs against it: Angular, or rangular.**

We want a clean rewrite of what the PHP e-commerce world built over two decades: catalog, cart, checkout, orders, payments, shipping, tax, promotions, multi-store, and admin. Not a clone for nostalgia's sake, but a **2027-ready** stack that keeps the good ideas and drops the legacy weight.

## The idea in one breath

| Piece           | What it is                                                                                                                                |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **Product**     | A **Rust** commerce API (domain, persistence, jobs, payments, webhooks)                                                                   |
| **UI option A** | **[Angular](https://angular.dev/)** (TypeScript SPA)                                                                                      |
| **UI option B** | **[rangular](https://github.com/Interchouette-ITC/rangular)** (Angular-like `.html` / `.scss`, Rust logic, rendered with **Leptos**/wasm) |

Same OpenAPI contract. Pick **one** storefront stack for a shop (or use both for different surfaces, e.g. Angular admin + rangular storefront). They are **two clients**, not two frameworks stacked on top of each other.

Leptos is not a third UI: it is the **browser render target** for the rangular path (`rangular` authoring → compile → Leptos DOM/wasm).

**Status:** vision and case study. No production code yet.

---

## Why this project exists

Open-source e-commerce still runs largely on PHP stacks. They work, they ship, and millions of shops depend on them. They also carry decades of extension models, security patches, hosting assumptions, and framework debt that make **greenfield features** expensive and **long-term operations** fragile.

**rustashop** asks a direct question:

> Can we rebuild a credible, extensible, open-source commerce platform in **Rust**, with modern web UIs against a single API, without pretending the last twenty years of merchant requirements never happened?

We think the answer is worth proving in public.

---

## The PHP landscape we are learning from (not copying)

These projects defined what "open-source shop software" means. Each solved real problems; each also shows where a Rust rewrite could help.

| Platform                                                                                      | Era / style                 | Strengths                                   | Pain we want to avoid                                   |
| --------------------------------------------------------------------------------------------- | --------------------------- | ------------------------------------------- | ------------------------------------------------------- |
| [osCommerce](https://www.oscommerce.com/)                                                     | Early 2000s monolith        | Simple mental model, huge extension history | Global state, SQL-in-templates, security surface        |
| [PrestaShop](https://www.prestashop.com/)                                                     | Module ecosystem, EU retail | Merchant features out of the box, themes    | Legacy core + module conflicts, upgrade fear            |
| [Magento / Adobe Commerce](https://business.adobe.com/products/magento/magento-commerce.html) | Enterprise PHP              | Catalog complexity, B2B, multi-store        | Heavy ops, slow iteration, extension brittleness        |
| [Sylius](https://sylius.com/)                                                                 | Symfony, API-first lean     | Clean domain boundaries, modern PHP         | Still PHP runtime costs, hosting model, ecosystem scale |
| [WooCommerce](https://woocommerce.com/)                                                       | WordPress plugin            | Distribution, content + commerce            | WP coupling, plugin matrix, performance at scale        |
| [OpenCart](https://www.opencart.com/)                                                         | Lightweight PHP             | Easy small-shop deploy                      | Same class of legacy patterns as peers                  |

We are **not** building "PrestaShop in Rust" or "Magento with Ferris." We are building a **commerce kernel** with explicit boundaries: catalog, pricing, inventory, cart, checkout, order fulfillment, customer accounts, promotions, content slots, and admin workflows. Themes and modules should attach to **stable interfaces**, not override core classes.

---

## Stack: one API, two UIs

### 1. Rust API (source of truth)

Commerce domain logic belongs in Rust:

- **Correctness:** money, tax, inventory, and promotions with strong types and explicit error paths.
- **Performance:** catalog search, cart recalculation, and checkout under load without tuning PHP-FPM pools first.
- **Operations:** single static binary or small container set, predictable memory.
- **Integrations:** payment providers, carriers, ERP, and webhooks as isolated crates/services.

Target shape (subject to change when coding starts):

- HTTP API (REST and/or GraphQL) with OpenAPI as the shared contract.
- PostgreSQL (or pluggable storage) for transactional data.
- Background workers for emails, webhooks, index rebuilds, and imports.
- Plugin/extension model via **WASM or stable Rust trait boundaries**, not monkey-patching core.

### 2. UI A: Angular (TypeScript)

Real **[Angular](https://angular.dev/)**: TypeScript, npm ecosystem, familiar SPA tooling.

Use this when the team already lives in Angular (agencies, large admin apps, third-party widgets, hiring pool).

### 3. UI B: rangular → Leptos (Rust / wasm)

**[rangular](https://github.com/Interchouette-ITC/rangular)** is a separate project: Angular-**like** templates (`.html`, `.scss`) with **Rust** hosts (`.rs`). Production builds lower to **[Leptos](https://leptos.dev/)** CSR/wasm in the browser.

Use this when you want one language from storefront widget to warehouse hook. Templates feel Angular-shaped; the runtime is **not** Angular.

|                 | UI A: Angular        | UI B: rangular                               |
| --------------- | -------------------- | -------------------------------------------- |
| Templates       | `.html`              | `.html` (Angular-like subset)                |
| Logic           | TypeScript           | Rust                                         |
| Styles          | `.scss`              | `.scss` (compiled in Rust; no Node for Sass) |
| Browser runtime | Angular / TypeScript | Leptos / wasm                                |
| Speaks to       | Same rustashop API   | Same rustashop API                           |

Live rangular demo: [rangular.interchouette.net](https://rangular.interchouette.net)

---

## High-level architecture (target)

```text
                    ┌─────────────────────────────────────┐
                    │           rustashop API             │
                    │  (Rust: catalog, cart, checkout,    │
                    │   orders, payments, admin, jobs)    │
                    └──────────────┬──────────────────────┘
                                   │  same OpenAPI contract
              ┌────────────────────┼────────────────────┐
              │                    │                    │
              ▼                    ▼                    ▼
     ┌────────────────┐  ┌────────────────┐  ┌────────────────┐
     │ UI A           │  │ UI B           │  │ Other clients  │
     │ Angular        │  │ rangular       │  │ (mobile, POS,  │
     │ (TypeScript)   │  │ → Leptos/wasm  │  │  headless)     │
     └────────────────┘  └────────────────┘  └────────────────┘
```

Pick UI A **or** UI B for a given storefront. Leptos only appears on path B, as rangular's render engine.

**Principles:**

1. **API-first:** every UI is a client; no hidden SQL in templates.
2. **Extension points, not core forks:** modules register hooks, do not replace internal classes.
3. **Money is never `f64`:** decimal types, currency rules, and tax jurisdictions are first-class.
4. **Upgrade path:** semver on API and extension manifest, with migration tooling from day one of v1.
5. **Self-host friendly:** Docker compose for small shops; horizontal scale for larger ones.

---

## Case study (2026): do we need a new open-source commerce stack in 2027?

_A structured look at necessity, not hype._

### Executive summary

**Necessary?** Not for every merchant. Existing PHP platforms still sell, ship, and pay bills.

**Worth doing anyway?** **Yes**, if the goal is a **credible open-source alternative** for teams who want:

- lower operational cost at scale,
- stronger safety around money and inventory,
- **one Rust API** with a choice of UI: Angular (TypeScript) **or** rangular (Rust / Leptos),
- and an extension model that survives major version upgrades.

2027 is a realistic horizon for a **focused MVP** (catalog, cart, checkout, orders, basic admin), not for feature parity with twenty years of Magento modules.

### Market context (2026)

| Factor                  | Reality                                                                                                                                                    |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SaaS dominance          | Shopify, BigCommerce, and hosted suites own the "easy" segment. Self-hosted open source is a **choice**, not the default.                                  |
| Headless / composable   | Merchants increasingly split CMS, PIM, OMS, and checkout. A Rust **API core** fits this trend better than another monolithic theme engine.                 |
| Security expectations   | Supply-chain audits, PCI scope, and patch cadence favor smaller, typed codebases with explicit boundaries.                                                 |
| Developer supply        | Rust backend hiring is growing; PHP module authors are aging out without replacements.                                                                     |
| AI-assisted development | Greenfield Rust + generated clients reduces the "rewrite is too big" argument, but **domain knowledge** (tax, shipping, promotions) remains the hard part. |

### Arguments **for** rebuilding in Rust (2027 target)

1. **Total cost of ownership:** predictable memory and CPU, fewer PHP workers to tune, simpler container images.
2. **Correctness under concurrency:** reservations, flash sales, and inventory decrements need explicit concurrency design; Rust forces that conversation early.
3. **Two UIs, one API:** Angular for TypeScript teams; [rangular](https://github.com/Interchouette-ITC/rangular) (+ Leptos) for Rust-native storefronts, without forking the commerce core.
4. **Extension safety:** WASM sandboxes or narrow FFI beats arbitrary PHP includes in `override/` folders.
5. **Long-term maintainability:** a young codebase with enforced lint gates vs. dragging forward 2005 patterns.

### Arguments **against** (honest)

1. **Ecosystem gap:** payment modules, carrier adapters, and ERP connectors exist by the thousand for PHP, not for a new Rust shop.
2. **Migration friction:** merchants will not rewrite themes for a pretty architecture; we need importers and parallel-run strategies.
3. **Sylius and API-first PHP** already prove that clean domain separation does not require leaving PHP.
4. **Time to parity:** promotions engines, multi-warehouse, B2B quotes, and marketplace modes are years of work.
5. **Risk of second-system syndrome:** rebuilding everything because the old stack is ugly, without a sharp MVP cut.

### Verdict for rustashop

| Question                                                           | Answer                                                                                                                                              |
| ------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Does the world **need** another open-source commerce core in 2027? | **Only if** it is narrowly scoped, API-first, and operatively cheaper than legacy self-host.                                                        |
| Is **Rust** the right backend bet?                                 | **Strong yes** for safety, ops, and long-running workers; **not magic** for product-market fit.                                                     |
| UI story?                                                          | **One API, two clients:** Angular (TypeScript) **or** rangular (Angular-like templates, Rust logic, Leptos/wasm). Not a mixed Angular+Leptos stack. |
| Should we chase Magento feature parity?                            | **No.** Ship catalog, cart, checkout, orders, payments, admin, and webhooks first.                                                                  |
| When to start coding?                                              | When this README's MVP scope has issues, milestones, and one payment provider chosen.                                                               |

**Bottom line:** rebuilding open-source commerce in Rust for 2027 is not a universal necessity, but it is a **credible opportunity** for self-hosted deployments, headless stacks, and teams that want to escape PHP legacy debt without leaving open source behind. **rustashop** exists to test that thesis in public.

---

## MVP scope (first proof, not the final product)

When implementation begins, v0.1 should prove the full vertical slice:

- [ ] Product catalog (variants, categories, media)
- [ ] Cart and session/checkout flow
- [ ] One payment provider (Stripe or Mollie class)
- [ ] Order persistence and basic admin (list, status, refund hook)
- [ ] Same API exercised by **both** clients: one Angular storefront **and** one rangular (→ Leptos) storefront
- [ ] Docker compose for local and small production deploy
- [ ] OpenAPI document published with every release

Explicitly **out of scope** for v0.1: multi-vendor marketplace, full promotion engine, native mobile apps, Magento import wizard.

---

## Relationship to [rangular](https://github.com/Interchouette-ITC/rangular)

**rangular** is upstream UI tooling (Angular-like templates for Leptos). **rustashop** consumes it as **UI option B**: real catalog, cart, and checkout screens as `.html` / `.scss` / `.rs` components.

Template language changes belong in rangular. Commerce domain belongs here.

---

## Contributing

The repository is intentionally light: discussion, issues, and ADRs welcome before the first `cargo init`.

1. Open an issue for domain topics (tax, shipping, plugin model) or which UI path to dogfood first.
2. Read the [rangular spec](https://github.com/Interchouette-ITC/rangular/blob/dev/docs/SPEC.md) for the wasm storefront path.
3. Keep commits and docs in **English**; conventional commits when code lands.

---

## License

To be decided before first release. Interchouette-ITC projects often use **Apache-2.0**; this repo will align with org policy when code is added.

---

## Links

| Resource           | URL                                            |
| ------------------ | ---------------------------------------------- |
| This repository    | https://github.com/Interchouette-ITC/rustashop |
| rangular           | https://github.com/Interchouette-ITC/rangular  |
| rangular live demo | https://rangular.interchouette.net             |
| Leptos             | https://leptos.dev/                            |
| Angular            | https://angular.dev/                           |

---

_rustashop: one Rust API; Angular or rangular on top; learn from PrestaShop, osCommerce, Magento, Sylius, and the rest._
