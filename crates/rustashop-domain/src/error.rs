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
    /// Quantity must be a positive integer.
    #[error("invalid quantity `{0}`")]
    InvalidQuantity(i32),
    /// Cart line id is not present on the cart.
    #[error("cart line not found `{0}`")]
    LineNotFound(String),
    /// Checkout requires at least one line.
    #[error("cart is empty")]
    EmptyCart,
    /// Cart was already converted to an order.
    #[error("cart already checked out")]
    CartAlreadyCheckedOut,
    /// Stored cart status is not `open` or `checked_out`.
    #[error("invalid cart status `{0}`")]
    InvalidCartStatus(String),
}
