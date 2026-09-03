//! Placed or draft order (`order` table; reserved SQL identifier).

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub number: String,
    pub customer_id: Option<Uuid>,
    pub cart_id: Option<Uuid>,
    pub state: String,
    pub currency: String,
    pub items_total_minor: i64,
    pub total_minor: i64,
    pub idempotency_key: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub placed_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
