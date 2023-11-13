use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AdminModel {
    pub id: Uuid,
    pub identification: String,
    pub identification_type: String,
    pub name: String,
    pub last_name: Option<String>,
    pub phonenumber: String,
    pub address: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub auth_user_id: Option<Uuid>,
}
