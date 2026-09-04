//! Order aggregate created at checkout.

use serde::{Deserialize, Serialize};

use crate::{Cart, CartLine, Currency, DomainError, Money};

/// Payment status before a payment provider is attached (`pending` at checkout).
pub const PAYMENT_STATUS_PENDING: &str = "pending";

/// Placed order line snapshot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderLine {
    /// Stable identifier.
    pub id: String,
    /// Parent order id.
    pub order_id: String,
    /// Variant id at checkout (optional if the SKU is later deleted).
    pub variant_id: Option<String>,
    /// Quantity greater than zero.
    pub quantity: i32,
    /// Snapshotted unit price.
    pub unit_price: Money,
    /// Snapshotted line total.
    pub line_total: Money,
    /// Snapshotted product display name.
    pub product_name: String,
    /// Snapshotted variant SKU.
    pub variant_sku: String,
}

impl OrderLine {
    fn from_cart_line(order_id: &str, line: &CartLine) -> Result<Self, DomainError> {
        Ok(Self {
            id: String::new(),
            order_id: order_id.to_owned(),
            variant_id: Some(line.variant_id.clone()),
            quantity: line.quantity,
            unit_price: line.unit_price.clone(),
            line_total: line.line_total()?,
            product_name: line.product_name.clone(),
            variant_sku: line.variant_sku.clone(),
        })
    }
}

/// Order created from a cart at checkout.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    /// Stable identifier.
    pub id: String,
    /// Human-readable order number.
    pub number: String,
    /// Source cart id.
    pub cart_id: Option<String>,
    /// Optional customer owner.
    pub customer_id: Option<String>,
    /// Fulfillment state (`placed` after checkout).
    pub state: String,
    /// Payment status (`pending` at checkout; no provider attached yet).
    pub payment_status: String,
    /// Order currency.
    pub currency: Currency,
    /// Sum of line totals.
    pub items_total: Money,
    /// Payable total (equals `items_total` until fees or taxes are modeled).
    pub total: Money,
    /// Optional client idempotency key.
    pub idempotency_key: Option<String>,
    /// Order lines.
    pub lines: Vec<OrderLine>,
}

impl Order {
    /// Builds a placed order from an open cart with lines.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyCart`], [`DomainError::CartAlreadyCheckedOut`],
    /// [`DomainError::CurrencyMismatch`], or [`DomainError::Overflow`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustashop_domain::{Cart, CartLine, CartStatus, Currency, Money, Order};
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
    /// let order = Order::from_cart(&cart, "o1".into(), "RS-1".into(), None).expect("place");
    /// assert_eq!(order.payment_status, "pending");
    /// assert_eq!(order.total.amount_minor, 200);
    /// ```
    pub fn from_cart(
        cart: &Cart,
        id: String,
        number: String,
        idempotency_key: Option<String>,
    ) -> Result<Self, DomainError> {
        cart.ensure_checkoutable()?;
        let items_total = cart.items_total()?;
        let mut lines = Vec::with_capacity(cart.lines.len());
        for line in &cart.lines {
            lines.push(OrderLine::from_cart_line(&id, line)?);
        }
        Ok(Self {
            id,
            number,
            cart_id: Some(cart.id.clone()),
            customer_id: cart.customer_id.clone(),
            state: "placed".to_owned(),
            payment_status: PAYMENT_STATUS_PENDING.to_owned(),
            currency: cart.currency.clone(),
            total: items_total.clone(),
            items_total,
            idempotency_key,
            lines,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CartStatus;

    #[test]
    fn empty_cart_cannot_check_out() {
        let eur = Currency::new("EUR").unwrap();
        let cart = Cart {
            id: "c1".into(),
            customer_id: None,
            token: "tok".into(),
            currency: eur,
            status: CartStatus::Open,
            lines: Vec::new(),
        };
        assert!(matches!(
            Order::from_cart(&cart, "o1".into(), "RS-1".into(), None),
            Err(DomainError::EmptyCart)
        ));
    }
}
