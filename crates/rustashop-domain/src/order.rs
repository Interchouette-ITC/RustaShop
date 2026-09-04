//! Order aggregate created at checkout.

use serde::{Deserialize, Serialize};

use crate::{Cart, CartLine, Currency, DomainError, Money};

/// Payment status before a payment provider is attached (`pending` at checkout).
pub const PAYMENT_STATUS_PENDING: &str = "pending";

/// Fulfillment lifecycle stored on `"order".state`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    /// Order placed at checkout.
    Placed,
    /// Marked paid by an operator (or payment flow later).
    Paid,
    /// Marked shipped by an operator.
    Shipped,
    /// Cancelled by an operator.
    Cancelled,
}

impl OrderState {
    /// Wire value stored in Postgres.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Placed => "placed",
            Self::Paid => "paid",
            Self::Shipped => "shipped",
            Self::Cancelled => "cancelled",
        }
    }

    /// Parses a stored or request status string.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidOrderState`] when `raw` is not a known state.
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        match raw {
            "placed" => Ok(Self::Placed),
            "paid" => Ok(Self::Paid),
            "shipped" => Ok(Self::Shipped),
            "cancelled" => Ok(Self::Cancelled),
            other => Err(DomainError::InvalidOrderState(other.to_owned())),
        }
    }
}

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
            state: OrderState::Placed.as_str().to_owned(),
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

    fn sample_cart(status: CartStatus) -> Cart {
        let eur = Currency::new("EUR").unwrap();
        Cart {
            id: "c1".into(),
            customer_id: Some("cust".into()),
            token: "tok".into(),
            currency: eur.clone(),
            status,
            lines: vec![CartLine {
                id: "l1".into(),
                cart_id: "c1".into(),
                variant_id: "v1".into(),
                quantity: 2,
                unit_price: Money::new(100, eur),
                product_name: "Mug".into(),
                variant_sku: "MUG".into(),
            }],
        }
    }

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

    #[test]
    fn checked_out_cart_cannot_check_out_again() {
        let cart = sample_cart(CartStatus::CheckedOut);
        assert!(matches!(
            Order::from_cart(&cart, "o1".into(), "RS-1".into(), None),
            Err(DomainError::CartAlreadyCheckedOut)
        ));
    }

    #[test]
    fn from_cart_places_pending_order() {
        let cart = sample_cart(CartStatus::Open);
        let order = Order::from_cart(&cart, "o1".into(), "RS-1".into(), Some("idem".into()))
            .expect("place");
        assert_eq!(order.id, "o1");
        assert_eq!(order.number, "RS-1");
        assert_eq!(order.cart_id.as_deref(), Some("c1"));
        assert_eq!(order.customer_id.as_deref(), Some("cust"));
        assert_eq!(order.state, "placed");
        assert_eq!(order.payment_status, PAYMENT_STATUS_PENDING);
        assert_eq!(order.total.amount_minor, 200);
        assert_eq!(order.items_total.amount_minor, 200);
        assert_eq!(order.idempotency_key.as_deref(), Some("idem"));
        assert_eq!(order.lines.len(), 1);
        assert_eq!(order.lines[0].variant_sku, "MUG");
        assert_eq!(order.lines[0].quantity, 2);
        assert_eq!(order.lines[0].line_total.amount_minor, 200);
    }

    #[rstest::rstest]
    #[case::placed(OrderState::Placed, "placed")]
    #[case::paid(OrderState::Paid, "paid")]
    #[case::shipped(OrderState::Shipped, "shipped")]
    #[case::cancelled(OrderState::Cancelled, "cancelled")]
    fn order_state_round_trips(#[case] state: OrderState, #[case] wire: &str) {
        assert_eq!(state.as_str(), wire);
        assert_eq!(OrderState::parse(wire).unwrap(), state);
    }

    #[test]
    fn order_state_parse_rejects_unknown() {
        assert!(matches!(
            OrderState::parse("bogus"),
            Err(DomainError::InvalidOrderState(_))
        ));
    }
}
