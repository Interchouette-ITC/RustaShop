# **rustashop**

**A modern open-source commerce platform: Rust on the server, Angular-shaped UI on the web.**

We want a clean, maintainable rewrite of what the PHP e-commerce world built over two decades: the catalog, cart, checkout, orders, payments, shipping, tax, promotions, multi-store, and admin back-office that merchants still depend on every day. Not a clone for nostalgia's sake, but a **2027-ready** stack that keeps the good ideas and drops the legacy weight.

| Layer                           | Technology                                                | Role                                                                       |
| ------------------------------- | --------------------------------------------------------- | -------------------------------------------------------------------------- |
| **Backend**                     | Rust                                                      | APIs, domain logic, persistence, jobs, integrations                        |
| **Storefront (classic)**        | [Angular](https://angular.dev/)                           | Full SPA where teams already live in TypeScript                            |
| **Storefront (Rust-native UI)** | [rangular](https://github.com/Interchouette-ITC/rangular) | Angular-shaped `.html` / `.scss` + Rust hosts, compiled to Leptos CSR/wasm |
| **Shared contract**             | OpenAPI / typed clients                                   | One commerce API, multiple frontends                                       |

**Status:** vision and case study. No production code yet. This repository is the home for the idea, the architecture notes, and (when we start) the Rust workspace.

---

## Why this project exists

Open-source e-commerce still runs largely on PHP stacks. They work, they ship, and millions of shops depend on them. They also carry decades of extension models, security patches, hosting assumptions, and framework debt that make **greenfield features** expensive and **long-term operations** fragile.

**rustashop** asks a direct question:

> Can we rebuild a credible, extensible, open-source commerce platform in **Rust + modern web UI**, without pretending the last twenty years of merchant requirements never happened?

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

## Stack: Rust + Angular + rangular

### Rust backend (source of truth)

Commerce domain logic belongs in Rust:

- **Correctness:** money, tax, inventory, and promotions are easier to reason about with strong types and explicit error paths.
- **Performance:** catalog search, cart recalculation, and checkout under load without tuning PHP-FPM pools first.
- **Operations:** single static binary or small container set, predictable memory, fewer moving parts on the server.
- **Integrations:** payment providers, carriers, ERP, and webhooks as isolated crates/services with clear failure modes.

Target shape (subject to change when coding starts):

- HTTP API (REST and/or GraphQL) with OpenAPI as the contract merchants and agencies can rely on.
- PostgreSQL (or pluggable storage) for transactional data.
- Background workers for emails, webhooks, index rebuilds, and imports.
- Plugin/extension model via **WASM or stable Rust trait boundaries**, not monkey-patching core.

### Angular storefront (ecosystem path)

Many agencies and in-house teams already standardize on **Angular** for large admin and storefront SPAs. A first-class **Angular** client against the rustashop API keeps adoption friction low: familiar routing, forms, i18n, and hiring pool.

Use Angular where **TypeScript velocity and npm ecosystem** matter most (rich admin, third-party widgets, legacy team skills).

### rangular storefront (Rust-native path)

[**rangular**](https://github.com/Interchouette-ITC/rangular) is Angular-**shaped** templates (`.html`, `.scss`) with **Rust controllers**, compiled to **Leptos** in the browser (CSR/wasm). Same component habit as Angular; logic stays in Rust.

|                     | Angular client              | rangular client                                        |
| ------------------- | --------------------------- | ------------------------------------------------------ |
| Template authoring  | `.html` + `.ts`             | `.html` + `.rs`                                        |
| Styles              | `.scss`                     | `.scss` (compiled in Rust via grass, no Node for Sass) |
| Runtime             | TypeScript in browser       | Rust to wasm + Leptos DOM                              |
| Best for            | Teams deep in Angular/npm   | Teams standardizing on Rust end-to-end                 |
| Shared with backend | API types (OpenAPI codegen) | Domain types and validation patterns                   |

Both frontends talk to the **same Rust API**. Merchants pick a theme stack; the platform does not force one UI religion.

Live rangular demo: [rangular.interchouette.net](https://rangular.interchouette.net)

---

## High-level architecture (target)

```text
                    ┌─────────────────────────────────────┐
                    │           rustashop API             │
                    │  (Rust: catalog, cart, checkout,    │
                    │   orders, payments, admin, jobs)    │
                    └──────────────┬──────────────────────┘
                                   │
              ┌────────────────────┼────────────────────┐
              │                    │                    │
              ▼                    ▼                    ▼
     ┌────────────────┐  ┌────────────────┐  ┌────────────────┐
     │ Angular        │  │ rangular       │  │ Headless       │
     │ storefront     │  │ storefront     │  │ clients        │
     │ + admin SPA    │  │ (Leptos/wasm)  │  │ (mobile, POS)  │
     └────────────────┘  └────────────────┘  └────────────────┘
```

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
- a UI story that spans **Angular shops** and **Rust-native wasm** via rangular,
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

1. **Total cost of ownership:** predictable memory and CPU, fewer PHP workers and opcode caches to tune, simpler container images.
2. **Correctness under concurrency:** reservations, flash sales, and inventory decrements need explicit concurrency design; Rust forces that conversation early.
3. **Dual frontend strategy:** Angular for ecosystem reach; [rangular](https://github.com/Interchouette-ITC/rangular) for teams that want one language from checkout widget to warehouse hook.
4. **Extension safety:** WASM sandboxes or narrow FFI beats arbitrary PHP includes in `override/` folders.
5. **Long-term maintainability:** a young codebase with enforced lint gates and fixture-tested UI templates (rangular habit) vs. dragging forward 2005 patterns.

### Arguments **against** (honest)

1. **Ecosystem gap:** payment modules, carrier adapters, and ERP connectors exist by the thousand for PHP, not for a new Rust shop.
2. **Migration friction:** merchants will not rewrite themes for a pretty architecture; we need importers and parallel-run strategies.
3. **Sylius and API-first PHP** already prove that clean domain separation does not require leaving PHP.
4. **Time to parity:** promotions engines, multi-warehouse, B2B quotes, and marketplace modes are years of work.
5. **Risk of second-system syndrome:** rebuilding everything because the old stack is ugly, without a sharp MVP cut.

### Verdict for rustashop

| Question                                                           | Answer                                                                                                            |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------- |
| Does the world **need** another open-source commerce core in 2027? | **Only if** it is narrowly scoped, API-first, and operatively cheaper than legacy self-host.                      |
| Is **Rust** the right backend bet?                                 | **Strong yes** for safety, ops, and long-running workers; **not magic** for product-market fit.                   |
| Is **Angular + rangular** the right UI bet?                        | **Yes as a pair:** Angular covers adoption; rangular covers Rust-native UX without abandoning familiar templates. |
| Should we chase Magento feature parity?                            | **No.** Ship catalog, cart, checkout, orders, payments, admin, and webhooks first.                                |
| When to start coding?                                              | When this README's MVP scope has issues, milestones, and one payment provider chosen.                             |

**Bottom line:** rebuilding open-source commerce in Rust for 2027 is not a universal necessity, but it is a **credible opportunity** for self-hosted deployments, headless stacks, and teams that want to escape PHP legacy debt without leaving open source behind. **rustashop** exists to test that thesis in public.

---

## MVP scope (first proof, not the final product)

When implementation begins, v0.1 should prove the full vertical slice:

- [ ] Product catalog (variants, categories, media)
- [ ] Cart and session/checkout flow
- [ ] One payment provider (Stripe or Mollie class)
- [ ] Order persistence and basic admin (list, status, refund hook)
- [ ] Storefront: one theme in **Angular** and one panel in **rangular** against the same API
- [ ] Docker compose for local and small production deploy
- [ ] OpenAPI document published with every release

Explicitly **out of scope** for v0.1: multi-vendor marketplace, full promotion engine, native mobile apps, Magento import wizard.

---

## Relationship to [rangular](https://github.com/Interchouette-ITC/rangular)

**rangular** is a separate project: a versioned subset of Angular-shaped templates for Leptos. **rustashop** is a **consumer** and proof case: real product lists, filters, cart lines, and checkout steps authored as `.html` / `.scss` / `.rs` components, dogfooding rangular the way a production shop theme would.

Contributions to template language belong upstream in rangular; commerce domain belongs here.

---

## Contributing

The repository is empty on purpose: discussion, issues, and ADRs welcome before the first `cargo init`.

1. Open an issue for domain topics (tax, shipping, plugin model) or UI strategy (Angular vs rangular themes).
2. Read the [rangular spec](https://github.com/Interchouette-ITC/rangular/blob/dev/docs/SPEC.md) if you care about the wasm storefront path.
3. Keep commits and docs in **English**; conventional commits when code lands.

---

## License

To be decided before first release. Interchouette-ITC projects often use **Apache-2.0**; this repo will align with org policy when code is added.

---

## Links

| Resource                         | URL                                            |
| -------------------------------- | ---------------------------------------------- |
| This repository                  | https://github.com/Interchouette-ITC/rustashop |
| rangular (UI templates for Rust) | https://github.com/Interchouette-ITC/rangular  |
| rangular live demo               | https://rangular.interchouette.net             |
| Angular                          | https://angular.dev/                           |

---

_rustashop: learn from PrestaShop, osCommerce, Magento, Sylius, and the rest; ship something merchants can host, developers can extend, and operators can sleep through._
