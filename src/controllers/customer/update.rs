use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::customer::CustomerModel, schema::customer::UpdateCustomerSchema, AppState};

// update
pub async fn update_customer_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateCustomerSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CustomerModel, "SELECT * FROM customers WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("customer not found")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let customer = query_result.unwrap();

    let query_result = sqlx::query_as!(
        CustomerModel,
        "UPDATE customers SET identification = $1, identification_type = $2, name = $3, last_name = $4, phonenumber = $5, address = $6, auth_user_id = $7, email = $8, updated_at = $9 WHERE id = $10 RETURNING *",
        body.identification.to_owned().unwrap_or(customer.identification),
        body.identification_type.to_owned().unwrap_or(customer.identification_type),
        body.name.to_owned().unwrap_or(customer.name),
        body.last_name,
        body.phonenumber.to_owned().unwrap_or(customer.phonenumber),
        body.address,
        body.auth_user_id,
        body.email,
        now,
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(customer) => {
            let customer_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "customer": customer
            })});

            return Ok(Json(customer_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
