use serde::{Deserialize, Serialize};

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
pub struct CreateProductSchema {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub stock: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductSchema {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub stock: Option<i32>,
}
