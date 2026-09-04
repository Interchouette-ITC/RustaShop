//! Cart aggregate and line snapshots.

use serde::{Deserialize, Serialize};

use crate::{Currency, DomainError, Money, ProductVariant};

/// Lifecycle of a shopping cart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CartStatus {
    /// Cart still accepts line mutations.
    Open,
    /// Cart was converted to an order.
    CheckedOut,
}

impl CartStatus {
    /// Wire value stored in Postgres.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::CheckedOut => "checked_out",
        }
    }

    /// Parses a stored status string.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidCartStatus`] when `raw` is not a known status.
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        match raw {
            "open" => Ok(Self::Open),
            "checked_out" => Ok(Self::CheckedOut),
            other => Err(DomainError::InvalidCartStatus(other.to_owned())),
        }
    }
}

/// Line on a shopping cart with price and label snapshots.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CartLine {
    /// Stable identifier.
    pub id: String,
    /// Parent cart id.
    pub cart_id: String,
    /// Purchased variant id.
    pub variant_id: String,
    /// Quantity greater than zero.
    pub quantity: i32,
    /// Snapshotted unit price.
    pub unit_price: Money,
    /// Snapshotted product display name.
    pub product_name: String,
    /// Snapshotted variant SKU.
    pub variant_sku: String,
}

impl CartLine {
    /// Builds a line from a live variant, product name, and quantity.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidQuantity`] when `quantity` is not positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustashop_domain::{CartLine, Currency, Money, ProductVariant};
    ///
    /// let eur = Currency::new("EUR").expect("valid");
    /// let variant = ProductVariant {
    ///     id: "v1".into(),
    ///     product_id: "p1".into(),
    ///     sku: "HOODIE-M".into(),
    ///     name: Some("Medium".into()),
    ///     price: Money::new(4500, eur),
    ///     stock_quantity: 3,
    /// };
    /// let line = CartLine::from_variant(
    ///     "l1".into(),
    ///     "c1".into(),
    ///     &variant,
    ///     "Hoodie".into(),
    ///     2,
    /// )
    /// .expect("qty");
    /// assert_eq!(line.line_total().expect("total").amount_minor, 9000);
    /// ```
    pub fn from_variant(
        id: String,
        cart_id: String,
        variant: &ProductVariant,
        product_name: String,
        quantity: i32,
    ) -> Result<Self, DomainError> {
        ensure_positive_quantity(quantity)?;
        Ok(Self {
            id,
            cart_id,
            variant_id: variant.id.clone(),
            quantity,
            unit_price: variant.price.clone(),
            product_name,
            variant_sku: variant.sku.clone(),
        })
    }

    /// Unit price × quantity in the line currency.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidQuantity`] or [`DomainError::Overflow`].
    pub fn line_total(&self) -> Result<Money, DomainError> {
        self.unit_price.checked_mul_qty(self.quantity)
    }

    /// Sets quantity and keeps the existing unit price snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidQuantity`] when `quantity` is not positive.
    pub fn set_quantity(&mut self, quantity: i32) -> Result<(), DomainError> {
        ensure_positive_quantity(quantity)?;
        self.quantity = quantity;
        Ok(())
    }
}

/// Shopping cart with snapshotted lines.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cart {
    /// Stable identifier.
    pub id: String,
    /// Optional customer owner.
    pub customer_id: Option<String>,
    /// Opaque session token.
    pub token: String,
    /// Cart currency; all lines must match.
    pub currency: Currency,
    /// Whether the cart is still open for mutation.
    pub status: CartStatus,
    /// Cart lines.
    pub lines: Vec<CartLine>,
}

impl Cart {
    /// Sum of line totals in the cart currency.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::CurrencyMismatch`] or [`DomainError::Overflow`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustashop_domain::{Cart, CartLine, CartStatus, Currency, Money};
    ///
    /// let eur = Currency::new("EUR").expect("valid");
    /// let cart = Cart {
    ///     id: "c1".into(),
    ///     customer_id: None,
    ///     token: "tok".into(),
    ///     currency: eur.clone(),
    ///     status: CartStatus::Open,
    ///     lines: vec![CartLine {
    ///         id: "l1".into(),
    ///         cart_id: "c1".into(),
    ///         variant_id: "v1".into(),
    ///         quantity: 2,
    ///         unit_price: Money::new(100, eur),
    ///         product_name: "Mug".into(),
    ///         variant_sku: "MUG".into(),
    ///     }],
    /// };
    /// assert_eq!(cart.items_total().expect("sum").amount_minor, 200);
    /// ```
    pub fn items_total(&self) -> Result<Money, DomainError> {
        let mut total = Money::new(0, self.currency.clone());
        for line in &self.lines {
            total = total.checked_add(&line.line_total()?)?;
        }
        Ok(total)
    }

    /// Adds a line or merges quantity when the variant already exists.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::CurrencyMismatch`] when the line currency differs,
    /// or [`DomainError::InvalidQuantity`] / [`DomainError::Overflow`] on merge.
    pub fn upsert_line(&mut self, line: CartLine) -> Result<(), DomainError> {
        if line.unit_price.currency != self.currency {
            return Err(DomainError::CurrencyMismatch {
                left: self.currency.as_str().to_owned(),
                right: line.unit_price.currency.as_str().to_owned(),
            });
        }
        if let Some(existing) = self
            .lines
            .iter_mut()
            .find(|row| row.variant_id == line.variant_id)
        {
            let quantity = existing
                .quantity
                .checked_add(line.quantity)
                .ok_or(DomainError::Overflow)?;
            existing.unit_price = line.unit_price;
            existing.product_name = line.product_name;
            existing.variant_sku = line.variant_sku;
            existing.set_quantity(quantity)?;
            return Ok(());
        }
        self.lines.push(line);
        Ok(())
    }

    /// Updates quantity for an existing line id.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::LineNotFound`] or [`DomainError::InvalidQuantity`].
    pub fn update_line_quantity(
        &mut self,
        line_id: &str,
        quantity: i32,
    ) -> Result<(), DomainError> {
        let line = self
            .lines
            .iter_mut()
            .find(|row| row.id == line_id)
            .ok_or_else(|| DomainError::LineNotFound(line_id.to_owned()))?;
        line.set_quantity(quantity)
    }

    /// Removes a line by id.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::LineNotFound`] when the id is unknown.
    pub fn remove_line(&mut self, line_id: &str) -> Result<(), DomainError> {
        let index = self
            .lines
            .iter()
            .position(|row| row.id == line_id)
            .ok_or_else(|| DomainError::LineNotFound(line_id.to_owned()))?;
        self.lines.remove(index);
        Ok(())
    }

    /// Rejects checkout when the cart has no lines or is already checked out.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyCart`] or [`DomainError::CartAlreadyCheckedOut`].
    pub fn ensure_checkoutable(&self) -> Result<(), DomainError> {
        if self.status == CartStatus::CheckedOut {
            return Err(DomainError::CartAlreadyCheckedOut);
        }
        if self.lines.is_empty() {
            return Err(DomainError::EmptyCart);
        }
        Ok(())
    }

    /// Marks the cart as converted to an order.
    pub const fn mark_checked_out(&mut self) {
        self.status = CartStatus::CheckedOut;
    }
}

const fn ensure_positive_quantity(quantity: i32) -> Result<(), DomainError> {
    if quantity > 0 {
        Ok(())
    } else {
        Err(DomainError::InvalidQuantity(quantity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Currency;

    fn eur_cart() -> Cart {
        Cart {
            id: "c1".to_owned(),
            customer_id: None,
            token: "tok".to_owned(),
            currency: Currency::new("EUR").unwrap(),
            status: CartStatus::Open,
            lines: Vec::new(),
        }
    }

    fn hoodie_variant() -> ProductVariant {
        ProductVariant {
            id: "v1".to_owned(),
            product_id: "p1".to_owned(),
            sku: "HOODIE-M".to_owned(),
            name: Some("Medium".to_owned()),
            price: Money::new(4500, Currency::new("EUR").unwrap()),
            stock_quantity: 8,
        }
    }

    #[test]
    fn upsert_merges_same_variant() {
        let mut cart = eur_cart();
        let line = CartLine::from_variant(
            "l1".to_owned(),
            cart.id.clone(),
            &hoodie_variant(),
            "Hoodie".to_owned(),
            1,
        )
        .unwrap();
        cart.upsert_line(line).unwrap();
        let again = CartLine::from_variant(
            "l2".to_owned(),
            cart.id.clone(),
            &hoodie_variant(),
            "Hoodie".to_owned(),
            2,
        )
        .unwrap();
        cart.upsert_line(again).unwrap();
        assert_eq!(cart.lines.len(), 1);
        assert_eq!(cart.lines[0].quantity, 3);
        assert_eq!(cart.items_total().unwrap().amount_minor, 13_500);
    }

    #[test]
    fn update_and_remove_line() {
        let mut cart = eur_cart();
        cart.upsert_line(
            CartLine::from_variant(
                "l1".to_owned(),
                cart.id.clone(),
                &hoodie_variant(),
                "Hoodie".to_owned(),
                1,
            )
            .unwrap(),
        )
        .unwrap();
        cart.update_line_quantity("l1", 4).unwrap();
        assert_eq!(cart.items_total().unwrap().amount_minor, 18_000);
        cart.remove_line("l1").unwrap();
        assert_eq!(cart.lines.len(), 0);
        assert_eq!(cart.items_total().unwrap().amount_minor, 0);
    }

    #[test]
    fn rejects_non_positive_quantity() {
        assert!(matches!(
            CartLine::from_variant(
                "l1".to_owned(),
                "c1".to_owned(),
                &hoodie_variant(),
                "Hoodie".to_owned(),
                0,
            ),
            Err(DomainError::InvalidQuantity(0))
        ));
    }
}
