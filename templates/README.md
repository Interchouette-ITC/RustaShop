# Templates

Designer-owned HTML/SCSS. Hosts only emit adapters; they do not own markup.

## Template kind (do not mix)

| Kind | Path | Hosts | Purpose |
| --- | --- | --- | --- |
| **shop** | `templates/shop/<id>/` | `shops/*` | Customer storefront |
| **admin** | `templates/admin/<id>/` | `admin/*` | Operator back-office |

Each package declares `rustashop.templateKind` in `package.json` (`"shop"` or `"admin"`).
Emit scripts refuse to run when the kind does not match the host.

Default ids today: `shop/default`, `admin/default`. Template **id** is renameable later;
**kind** is the stable split (same idea as keeping admin URL renameable separately).

## Packages

| Path | npm | Cargo |
| --- | --- | --- |
| `templates/shop/default` | `@rustashop/template-shop-default` | `rustashop-template-shop-default` |
| `templates/admin/default` | `@rustashop/template-admin-default` | (Angular sample only for now) |

Do not put admin panels under `templates/shop/`, or shop chrome under `templates/admin/`.
