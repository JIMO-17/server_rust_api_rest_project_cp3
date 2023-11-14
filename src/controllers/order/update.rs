use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::order::OrderModel, schema::order::UpdateOrderSchema, AppState};

// update note
pub async fn update_order_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateOrderSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(OrderModel, "SELECT * FROM orders WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("order with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let order = query_result.unwrap();

    let query_result = sqlx::query_as!(
        OrderModel,
        "UPDATE orders SET quantity = $1, total = $2, admin_id = $3, customer_id = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.quantity.to_owned().unwrap_or(order.quantity),
        body.total.to_owned().unwrap_or(order.total),
        body.admin_id,
        body.customer_id,
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(order) => {
            let order_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "order": order
            })});

            return Ok(Json(order_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
