use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{
    models::customer::CustomerModel, schema::customer::CreateCustomerSchema, AppState,
};

pub async fn create_customer_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCustomerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CustomerModel,
        "INSERT INTO customers (identification,identification_type,name,last_name,phonenumber,address,auth_user_id, email) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        body.identification.to_string(),
        body.identification_type.to_string(),
        body.name.to_string(),
        body.last_name,
        body.phonenumber.to_string(),
        body.address,
        body.auth_user_id,
        body.email,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(customer) => {
            let customer_response = json!({"status": "success","data": json!({
                "customer": customer
            })});

            return Ok((StatusCode::CREATED, Json(customer_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "customer already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}
