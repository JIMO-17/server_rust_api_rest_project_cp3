use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
    pub quantity: i32,
    pub total: i32,
    pub customer_id: Option<Uuid>,
    pub admin_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateOrderSchema {
    pub quantity: Option<i32>,
    pub total: Option<i32>,
    pub customer_id: Option<Uuid>,
    pub admin_id: Option<Uuid>,
}
