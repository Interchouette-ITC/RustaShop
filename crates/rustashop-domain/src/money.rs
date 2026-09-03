//! Integer minor-unit money.

use serde::{Deserialize, Serialize};

use crate::DomainError;

/// ISO 4217 alphabetic currency code (exactly three ASCII letters).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency(String);

impl Currency {
    /// Parses a three-letter currency code (ASCII case-insensitive).
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidCurrency`] when the code is not three letters.
    pub fn new(code: impl Into<String>) -> Result<Self, DomainError> {
        let code = code.into().to_ascii_uppercase();
        if code.len() == 3 && code.chars().all(|c| c.is_ascii_alphabetic()) {
            Ok(Self(code))
        } else {
            Err(DomainError::InvalidCurrency(code))
        }
    }

    /// Currency code as uppercase text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Money amount in minor units for a single currency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// Amount in the currency's minor unit (for example cents).
    pub amount_minor: i64,
    /// ISO currency for `amount_minor`.
    pub currency: Currency,
}

impl Money {
    /// Creates money in `currency` without changing the minor-unit scale.
    #[must_use]
    pub const fn new(amount_minor: i64, currency: Currency) -> Self {
        Self {
            amount_minor,
            currency,
        }
    }

    /// Adds two amounts of the same currency.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::CurrencyMismatch`] or [`DomainError::Overflow`].
    pub fn checked_add(&self, other: &Self) -> Result<Self, DomainError> {
        self.ensure_same_currency(other)?;
        let amount_minor = self
            .amount_minor
            .checked_add(other.amount_minor)
            .ok_or(DomainError::Overflow)?;
        Ok(Self::new(amount_minor, self.currency.clone()))
    }

    /// Subtracts `other` from this amount.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::CurrencyMismatch`] or [`DomainError::Overflow`].
    pub fn checked_sub(&self, other: &Self) -> Result<Self, DomainError> {
        self.ensure_same_currency(other)?;
        let amount_minor = self
            .amount_minor
            .checked_sub(other.amount_minor)
            .ok_or(DomainError::Overflow)?;
        Ok(Self::new(amount_minor, self.currency.clone()))
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), DomainError> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(DomainError::CurrencyMismatch {
                left: self.currency.as_str().to_owned(),
                right: other.currency.as_str().to_owned(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_requires_three_letters() {
        assert!(Currency::new("eur").is_ok());
        assert!(matches!(
            Currency::new("eu"),
            Err(DomainError::InvalidCurrency(_))
        ));
        assert!(matches!(
            Currency::new("eu1"),
            Err(DomainError::InvalidCurrency(_))
        ));
    }

    #[test]
    fn add_and_sub_same_currency() {
        let eur = Currency::new("EUR").unwrap();
        let left = Money::new(199, eur.clone());
        let right = Money::new(50, eur);
        assert_eq!(left.checked_add(&right).unwrap().amount_minor, 249);
        assert_eq!(left.checked_sub(&right).unwrap().amount_minor, 149);
    }

    #[test]
    fn currency_mismatch_is_error() {
        let left = Money::new(100, Currency::new("EUR").unwrap());
        let right = Money::new(100, Currency::new("USD").unwrap());
        assert!(matches!(
            left.checked_add(&right),
            Err(DomainError::CurrencyMismatch { .. })
        ));
    }

    #[test]
    fn overflow_does_not_panic() {
        let eur = Currency::new("EUR").unwrap();
        let left = Money::new(i64::MAX, eur.clone());
        let right = Money::new(1, eur);
        assert_eq!(left.checked_add(&right), Err(DomainError::Overflow));
        assert_eq!(
            Money::new(i64::MIN, Currency::new("EUR").unwrap()).checked_sub(&right),
            Err(DomainError::Overflow)
        );
    }
}
