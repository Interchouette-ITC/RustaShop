# Domains and deploy surfaces

Operator creates Render services; **no Render Blueprint / service YAML is maintained in this repo** for that hand-off. This note is the product map so DNS, images, and clients stay aligned.

## Hostnames

| Host | Role |
| --- | --- |
| `rustashop.interchouette.net` | First **`:dev` image** deploy (API / stack tip while building) |
| `rustashop.ai` | **Primary** marketing and public brand |
| `rustashop.io` | **Product**-oriented surface (docs, product home, API-facing story) |
| `rustashop.dev` | **Demo** shop / playground |
| `rustashop.app` | **Ionic** (or mobile) app entry |
| `rustashop.nl` | Redirect → `rustashop.ai` (for now) |
| `rustashop.eu` | Redirect → `rustashop.ai` (for now) |
| `rustashop.fr` | Redirect → `rustashop.ai` (for now) |

## Deploy order (intent)

1. Ship a **dev image** and attach it to `rustashop.interchouette.net` (Render service owned by the operator; MCP/ops may assist image publish elsewhere).
2. Point marketing at `rustashop.ai`.
3. Stand up `rustashop.io` / `rustashop.dev` when product and demo builds exist.
4. `rustashop.app` follows the mobile client.
5. Keep `.nl` / `.eu` / `.fr` as redirects until localized sites are justified.

## Client mapping (later)

| Surface | Likely client |
| --- | --- |
| Marketing (`ai`) | Static / Angular marketing |
| Product (`io`) | Docs + product pages |
| Demo (`dev`) | Full storefront against demo API |
| App (`app`) | Ionic |
| Tip (`interchouette.net` subdomain) | Dev API / preview |

Exact Compose/image names land with the ops epic when Docker exists. Do not invent Render service files here.
