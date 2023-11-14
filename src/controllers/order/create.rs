use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{models::order::OrderModel, schema::order::CreateOrderSchema, AppState};

pub async fn create_order_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateOrderSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        OrderModel,
        "INSERT INTO orders (quantity, total, customer_id, admin_id) VALUES ($1, $2, $3, $4) RETURNING *",
        body.quantity,
        body.total,
        body.customer_id,
        body.admin_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(order) => {
            let order_response = json!({"status": "success","data": json!({
                "order": order
            })});

            return Ok((StatusCode::CREATED, Json(order_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "order with that title already exists",
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
