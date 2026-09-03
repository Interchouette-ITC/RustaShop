//! Deny `format!(…)` that builds SQL in persist adapter sources.
//!
//! Run via `cargo test -p rustashop-persist-sqlx --test sql_safety` or `make check-sql-safety`.

use std::fs;
use std::path::{Path, PathBuf};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn line_builds_sql_with_format(line: &str) -> bool {
    let Some(start) = line.find("format!(") else {
        return false;
    };
    let rest = line[start..].to_ascii_uppercase();
    rest.contains("SELECT")
        || rest.contains("INSERT")
        || rest.contains("UPDATE")
        || rest.contains("DELETE")
        || rest.contains("WHERE")
        || rest.contains("FROM ")
}

fn scan_rust_sources(dir: &Path, violations: &mut Vec<String>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|error| {
        panic!("read_dir {}: {error}", dir.display());
    });
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            scan_rust_sources(&path, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("read {}: {error}", path.display());
        });
        for (index, line) in text.lines().enumerate() {
            if line_builds_sql_with_format(line) {
                violations.push(format!("{}:{}:{line}", path.display(), index + 1));
            }
        }
    }
}

#[test]
fn deny_format_built_sql_in_persist_crates() {
    let root = workspace_root();
    let mut violations = Vec::new();
    for relative in [
        "crates/rustashop-persist-sqlx/src",
        "crates/rustashop-persist-seaorm/src",
    ] {
        scan_rust_sources(&root.join(relative), &mut violations);
    }
    assert!(
        violations.is_empty(),
        "format!-built SQL is forbidden in persist crates:\n{}",
        violations.join("\n")
    );
}

#[test]
fn deny_pattern_matches_fixture_and_allows_non_sql() {
    assert!(line_builds_sql_with_format(
        r#"let q = format!("SELECT * FROM t WHERE id = '{id}');"#
    ));
    assert!(!line_builds_sql_with_format(
        r#"format!("invalid id `{id}`")"#
    ));
    assert!(!line_builds_sql_with_format(r#"format!("RS-{}", suffix)"#));
}
