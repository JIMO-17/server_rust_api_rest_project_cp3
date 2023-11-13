use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{models::product::ProductModel, schema::product::CreateProductSchema, AppState};

pub async fn create_product_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateProductSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ProductModel,
        "INSERT INTO products (name, description, price, stock) VALUES ($1, $2, $3, $4) RETURNING *",
        body.name.to_string(),
        body.description.to_string(),
        body.price,
        body.stock
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(product) => {
            let product_response = json!({"status": "success","data": json!({
                "product": product
            })});

            return Ok((StatusCode::CREATED, Json(product_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "product with that title already exists",
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
