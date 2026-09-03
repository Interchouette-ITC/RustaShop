//! Persist-param hygiene before bind / filter (NUL rejection).
//!
//! Not SQL-injection protection; parameterized queries remain required.

use serenade_contracts::{reject_unsafe_sql_param, PersistenceError};

/// Rejects NUL in a string that will be bound or filtered.
pub fn ensure_param(value: &str) -> Result<&str, PersistenceError> {
    reject_unsafe_sql_param(value)
}

/// Optional string parameter.
pub fn ensure_param_opt<S: AsRef<str> + ?Sized>(
    value: Option<&S>,
) -> Result<Option<&str>, PersistenceError> {
    match value {
        None => Ok(None),
        Some(value) => Ok(Some(ensure_param(value.as_ref())?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nul() {
        assert!(ensure_param("a\0b").is_err());
    }

    #[test]
    fn accepts_clean() {
        assert_eq!(ensure_param("ok").unwrap(), "ok");
        assert_eq!(ensure_param_opt(None::<&str>).unwrap(), None);
        let owned = String::from("id");
        assert_eq!(ensure_param_opt(Some(&owned)).unwrap(), Some("id"));
    }
}
