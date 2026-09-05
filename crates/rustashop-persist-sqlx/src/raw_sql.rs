//! Gate for raw SQL text that is not a static migration/seed script.
//!
//! Client-supplied SQL fragments are disabled unless `RUSTASHOP_ALLOW_RAW_SQL`
//! is explicitly enabled. Parameterized queries remain the only normal path.

use crate::param::ensure_param;
use serenade_contracts::PersistenceError;
use sqlx::postgres::PgPool;

/// Env var that enables raw client SQL fragments (default: off).
pub const ALLOW_RAW_SQL_ENV: &str = "RUSTASHOP_ALLOW_RAW_SQL";

/// Returns whether raw client SQL is allowed for this process.
#[must_use]
pub fn raw_sql_allowed() -> bool {
    std::env::var(ALLOW_RAW_SQL_ENV).is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

/// Ensures [`ALLOW_RAW_SQL_ENV`] is set; otherwise returns [`PersistenceError::InvalidInput`].
///
/// # Errors
///
/// When the flag is unset or not a recognized enable value.
pub fn assert_raw_sql_allowed() -> Result<(), PersistenceError> {
    if raw_sql_allowed() {
        Ok(())
    } else {
        Err(PersistenceError::InvalidInput {
            message: format!(
                "raw client SQL is disabled; set {ALLOW_RAW_SQL_ENV}=1 only if you accept that risk"
            ),
        })
    }
}

/// Executes a raw SQL fragment when [`raw_sql_allowed`] is true.
///
/// Static migrations and [`crate::seed_catalog`] do **not** use this path.
/// Even when enabled, the fragment must not contain NUL.
///
/// # Errors
///
/// Flag off, NUL in `sql`, or driver failure.
pub async fn execute_fragment(pool: &PgPool, sql: &str) -> Result<(), PersistenceError> {
    assert_raw_sql_allowed()?;
    ensure_param(sql)?;
    eprintln!("WARNING: {ALLOW_RAW_SQL_ENV} enabled; executing raw SQL fragment");
    sqlx::query(sqlx::AssertSqlSafe(sql.to_owned()))
        .execute(pool)
        .await
        .map_err(|error| PersistenceError::Internal {
            message: error.to_string(),
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn raw_sql_gate_and_execute_fragment() {
        // Single async test: parallel `remove_var` / `set_var` across two tests
        // raced under CI (`--test-threads` > 1) and flaked locally vs Actions.
        unsafe {
            std::env::remove_var(ALLOW_RAW_SQL_ENV);
        }
        assert!(!raw_sql_allowed());
        assert!(assert_raw_sql_allowed().is_err());

        unsafe {
            std::env::set_var(ALLOW_RAW_SQL_ENV, "1");
        }
        assert!(raw_sql_allowed());
        assert!(assert_raw_sql_allowed().is_ok());

        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .expect("connect");
        execute_fragment(&pool, "SELECT 1").await.expect("execute");
        assert!(execute_fragment(&pool, "SELECT 1\0").await.is_err());
        let bad = execute_fragment(&pool, "SELECT FROM")
            .await
            .expect_err("bad sql");
        assert!(matches!(bad, PersistenceError::Internal { .. }));

        unsafe {
            std::env::remove_var(ALLOW_RAW_SQL_ENV);
        }
        assert!(!raw_sql_allowed());
        assert!(execute_fragment(&pool, "SELECT 1").await.is_err());
    }
}
