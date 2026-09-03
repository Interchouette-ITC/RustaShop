#!/usr/bin/env bash
# Deny format!-built SQL with keywords in persist adapter crates.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PATTERN='format!\([^)]*(SELECT|INSERT|UPDATE|DELETE|WHERE|FROM\s)'
TARGETS=(
  crates/rustashop-persist-sqlx/src
  crates/rustashop-persist-seaorm/src
)

fail=0

if command -v rg >/dev/null 2>&1; then
  if rg -n --pcre2 "$PATTERN" "${TARGETS[@]}" 2>/dev/null; then
    echo "check-sql-safety: found format! building SQL in persist crates" >&2
    fail=1
  fi
else
  if grep -REn "$PATTERN" "${TARGETS[@]}" 2>/dev/null; then
    echo "check-sql-safety: found format! building SQL in persist crates" >&2
    fail=1
  fi
fi

# Self-test: the deny pattern must match a planted concat-SQL line.
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
printf '%s\n' 'let q = format!("SELECT * FROM t WHERE id = '\''{id}'\''");' >"$tmp"
if command -v rg >/dev/null 2>&1; then
  if ! rg -q --pcre2 "$PATTERN" "$tmp"; then
    echo "check-sql-safety: self-test failed (deny pattern did not match fixture)" >&2
    fail=1
  fi
elif ! grep -Eq "$PATTERN" "$tmp"; then
  echo "check-sql-safety: self-test failed (deny pattern did not match fixture)" >&2
  fail=1
fi

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi

echo "check-sql-safety: ok"
