use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::auth_user::AuthUserModel, schema::auth_user::UpdateAuthUserSchema, AppState};

// update auth_user
pub async fn update_auth_user_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateAuthUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(AuthUserModel, "SELECT * FROM auth_users WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("User not found")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let auth_user = query_result.unwrap();

    let query_result = sqlx::query_as!(
        AuthUserModel,
        "UPDATE auth_users SET email = $1, active = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        body.email.to_owned().unwrap_or(auth_user.email),
        body.active,
        now,
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(auth_user) => {
            let auth_user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "auth_user": auth_user
            })});

            return Ok(Json(auth_user_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
