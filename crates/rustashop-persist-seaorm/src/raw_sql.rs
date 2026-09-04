//! Gate for raw SQL text that is not a static migration/seed script.
//!
//! Client-supplied SQL fragments are disabled unless `RUSTASHOP_ALLOW_RAW_SQL`
//! is explicitly enabled.

use crate::param::ensure_param;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use serenade_contracts::PersistenceError;

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

/// Ensures [`ALLOW_RAW_SQL_ENV`] is set.
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
/// # Errors
///
/// Flag off, NUL in `sql`, or driver failure.
pub async fn execute_fragment(
    connection: &DatabaseConnection,
    sql: &str,
) -> Result<(), PersistenceError> {
    assert_raw_sql_allowed()?;
    ensure_param(sql)?;
    eprintln!("WARNING: {ALLOW_RAW_SQL_ENV} enabled; executing raw SQL fragment");
    connection
        .execute_raw(Statement::from_string(
            connection.get_database_backend(),
            sql.to_owned(),
        ))
        .await
        .map_err(|error| PersistenceError::Internal {
            message: error.to_string(),
        })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_sql_flag_round_trip() {
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
        unsafe {
            std::env::remove_var(ALLOW_RAW_SQL_ENV);
        }
        assert!(!raw_sql_allowed());
    }

    #[tokio::test]
    async fn execute_fragment_when_allowed() {
        let url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
        unsafe {
            std::env::set_var(ALLOW_RAW_SQL_ENV, "true");
        }
        let mut options = sea_orm::ConnectOptions::new(url);
        options.max_connections(1);
        let db = sea_orm::Database::connect(options).await.expect("connect");
        execute_fragment(&db, "SELECT 1").await.expect("execute");
        assert!(execute_fragment(&db, "SELECT 1\0").await.is_err());
        unsafe {
            std::env::remove_var(ALLOW_RAW_SQL_ENV);
        }
        assert!(execute_fragment(&db, "SELECT 1").await.is_err());
    }
}
