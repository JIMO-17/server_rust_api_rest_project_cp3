use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::customer::CustomerModel, AppState};

// get by id
pub async fn get_customer_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CustomerModel, "SELECT * FROM customers WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(customer) => {
            let customer_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "customer": customer
            })});

            return Ok(Json(customer_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("customer not found")
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
