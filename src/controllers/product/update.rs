use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::product::ProductModel, schema::product::UpdateProductSchema, AppState};

// update note
pub async fn update_product_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateProductSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ProductModel, "SELECT * FROM products WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("product with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let product = query_result.unwrap();

    let query_result = sqlx::query_as!(
        ProductModel,
        "UPDATE products SET name = $1, description = $2, price = $3, stock = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.name.to_owned().unwrap_or(product.name),
        body.description.to_owned().unwrap_or(product.description),
        body.price.to_owned().unwrap_or(product.price),
        body.stock.unwrap_or(product.stock),
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "product": product
            })});

            return Ok(Json(product_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
