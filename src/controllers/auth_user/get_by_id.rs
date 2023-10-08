use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::auth_user::AuthUserModel, AppState};

// get by id
pub async fn get_auth_user_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(AuthUserModel, "SELECT * FROM auth_users WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(auth_user) => {
            let auth_user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "auth_user": {
                    "id": auth_user.id,
                    "email": auth_user.email,
                    "active": auth_user.active,
                }
            })});

            return Ok(Json(auth_user_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User not found")
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
