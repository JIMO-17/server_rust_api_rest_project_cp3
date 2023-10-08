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
pub struct CreateAuthUsetSchema {
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAuthUserSchema {
    pub email: Option<String>,
    pub password: Option<String>,
    pub active: Option<bool>,
}