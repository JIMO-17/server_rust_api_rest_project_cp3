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
pub struct CreateAdminSchema {
    pub identification: String,
    pub identification_type: String,
    pub name: String,
    pub last_name: Option<String>,
    pub phonenumber: String,
    pub address: Option<String>,
    pub auth_user_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAdminSchema {
    pub identification: Option<String>,
    pub identification_type: Option<String>,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub phonenumber: Option<String>,
    pub address: Option<String>,
    pub auth_user_id: Option<Uuid>,
}
