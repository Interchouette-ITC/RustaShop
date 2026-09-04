//! Commerce API client (same `/api` prefix as the Angular shop; Trunk proxies to Actix).

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "/api";

#[derive(Clone, Debug, Deserialize)]
pub struct Money {
    pub amount_minor: i64,
    pub currency: String,
}

impl Money {
    #[must_use]
    pub fn display(&self) -> String {
        let major = self.amount_minor / 100;
        let cents = self.amount_minor.rem_euclid(100);
        format!("{major}.{cents:02} {}", self.currency)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct Product {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProductListResponse {
    pub items: Vec<Product>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct ProductVariant {
    pub id: String,
    pub product_id: String,
    pub sku: String,
    pub name: Option<String>,
    pub price: Money,
    pub stock_quantity: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct ProductDetail {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub variants: Vec<ProductVariant>,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct CartLine {
    pub id: String,
    pub variant_id: String,
    pub quantity: i32,
    pub unit_price: Money,
    pub line_total: Money,
    pub product_name: String,
    pub variant_sku: String,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct Cart {
    pub id: String,
    pub status: String,
    pub currency: String,
    pub lines: Vec<CartLine>,
    pub items_total: Money,
}

#[derive(Serialize)]
struct AddLineBody<'a> {
    variant_id: &'a str,
    quantity: i32,
}

fn url(path: &str) -> String {
    format!("{API_BASE}{path}")
}

async fn read_json<T: for<'de> Deserialize<'de>>(resp: gloo_net::http::Response) -> Result<T, String> {
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<T>()
        .await
        .map_err(|err| format!("decode: {err}"))
}

pub async fn list_products() -> Result<ProductListResponse, String> {
    let resp = Request::get(&url("/v1/products"))
        .send()
        .await
        .map_err(|err| format!("network: {err}"))?;
    read_json(resp).await
}

pub async fn get_product(id: &str) -> Result<ProductDetail, String> {
    let resp = Request::get(&url(&format!("/v1/products/{id}")))
        .send()
        .await
        .map_err(|err| format!("network: {err}"))?;
    read_json(resp).await
}

pub async fn create_cart() -> Result<Cart, String> {
    let resp = Request::post(&url("/v1/carts"))
        .header("content-type", "application/json")
        .body("{}")
        .map_err(|err| format!("body: {err}"))?
        .send()
        .await
        .map_err(|err| format!("network: {err}"))?;
    read_json(resp).await
}

pub async fn get_cart(id: &str) -> Result<Cart, String> {
    let resp = Request::get(&url(&format!("/v1/carts/{id}")))
        .send()
        .await
        .map_err(|err| format!("network: {err}"))?;
    read_json(resp).await
}

pub async fn add_cart_line(cart_id: &str, variant_id: &str, quantity: i32) -> Result<Cart, String> {
    let body = serde_json::to_string(&AddLineBody {
        variant_id,
        quantity,
    })
    .map_err(|err| format!("encode: {err}"))?;
    let resp = Request::post(&url(&format!("/v1/carts/{cart_id}/lines")))
        .header("content-type", "application/json")
        .body(body)
        .map_err(|err| format!("body: {err}"))?
        .send()
        .await
        .map_err(|err| format!("network: {err}"))?;
    read_json(resp).await
}
