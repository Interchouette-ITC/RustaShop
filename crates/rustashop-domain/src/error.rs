//! Domain error types.

/// Failure in money or catalog value objects.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Currency code is not a three-letter ISO alphabetic code.
    #[error("invalid currency code `{0}`")]
    InvalidCurrency(String),
    /// Arithmetic mixed two different currencies.
    #[error("currency mismatch: {left} vs {right}")]
    CurrencyMismatch {
        /// Left-hand currency code.
        left: String,
        /// Right-hand currency code.
        right: String,
    },
    /// Checked arithmetic overflowed `i64`.
    #[error("money amount overflow")]
    Overflow,
}
