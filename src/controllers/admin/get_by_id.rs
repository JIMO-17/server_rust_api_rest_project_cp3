use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::admin::AdminModel, AppState};

// get by id
pub async fn get_admin_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(AdminModel, "SELECT * FROM admins WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(admin) => {
            let auth_user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "admin": admin
            })});

            return Ok(Json(auth_user_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Admin not found")
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
