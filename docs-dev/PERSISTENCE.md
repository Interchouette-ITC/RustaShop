# Persistence SQL safety

Adapters in `rustashop-persist-sqlx` and `rustashop-persist-seaorm` use **parameterized** queries (SQLx binds) or the SeaORM query builder. String concatenation must not build SQL from request or domain data.

## Persist-param hygiene

Before bind or filter, string inputs (ids, slugs, tokens, currency codes, product names copied into lines, idempotency keys) pass through `param::ensure_param`, which rejects **NUL** (`\0`) via the shared contracts helper. That is input hygiene, not a substitute for binds.

## Raw client SQL

There is no public HTTP handler that accepts a client SQL string. Internal escape hatch:

| Item | Behavior |
| --- | --- |
| Env | `RUSTASHOP_ALLOW_RAW_SQL` (default unset / off) |
| Entry | `raw_sql::execute_fragment` in each persist crate |
| Flag off | `PersistenceError::InvalidInput` |
| Flag on | Loud stderr warning, then execute (still reject NUL) |

Static migrations and catalog seed scripts stay outside that gate.

## Lint

`make check-sql-safety` (also run from `make lint`) fails if persist crate sources use `format!(…)` with SQL keywords to build statements.
