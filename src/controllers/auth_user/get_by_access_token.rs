use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::auth_user::AuthUserModel, AppState};

// get access token to verify signed in user
pub async fn get_auth_user_by_access_token_handler(
    Path(access_token): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        AuthUserModel,
        "SELECT * FROM auth_users WHERE access_token = $1",
        access_token
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(auth_user) => {
            let auth_user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "access_token": auth_user.access_token,
            })});

            return Ok(Json(auth_user_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("User has no access token")
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}
