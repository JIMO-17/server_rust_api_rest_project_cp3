use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::product::ProductModel, AppState};

// get by id
pub async fn get_product_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ProductModel, "SELECT * FROM products WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(product) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "product": product
            })});

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("product with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
