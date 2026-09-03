//! Catalog aggregates: category, product, and purchasable variant.

use serde::{Deserialize, Serialize};

use crate::Money;

/// Category tree node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    /// Stable identifier.
    pub id: String,
    /// Optional parent category id.
    pub parent_id: Option<String>,
    /// URL slug unique within the parent.
    pub slug: String,
    /// Display name.
    pub name: String,
}

/// Sellable product.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Product {
    /// Stable identifier.
    pub id: String,
    /// Optional category id.
    pub category_id: Option<String>,
    /// Unique URL slug.
    pub slug: String,
    /// Display name.
    pub name: String,
    /// Optional long description.
    pub description: Option<String>,
    /// Whether the product is offered for sale.
    pub enabled: bool,
}

/// Purchasable SKU with integer minor-unit price.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductVariant {
    /// Stable identifier.
    pub id: String,
    /// Parent product id.
    pub product_id: String,
    /// Unique stock-keeping unit.
    pub sku: String,
    /// Optional variant label.
    pub name: Option<String>,
    /// Unit price in minor units.
    pub price: Money,
    /// Available stock quantity.
    pub stock_quantity: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Currency;

    #[test]
    fn product_variant_serde_roundtrip() {
        let variant = ProductVariant {
            id: "v1".to_owned(),
            product_id: "p1".to_owned(),
            sku: "HOODIE-M".to_owned(),
            name: Some("Medium".to_owned()),
            price: Money::new(4500, Currency::new("EUR").unwrap()),
            stock_quantity: 3,
        };
        let json = serde_json::to_string(&variant).unwrap();
        let parsed: ProductVariant = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, variant);
    }
}
