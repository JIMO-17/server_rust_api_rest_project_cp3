use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{
    models::auth_user::AuthUserModel, schema::auth_user::CreateAuthUsetSchema,
    utils::fn_password::hash_password, AppState,
};

pub async fn create_auth_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateAuthUsetSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if body.email.is_empty() || body.password.is_empty() {
        let error_response = serde_json::json!({
            "status": "failed",
            "message": "Email and password are required",
        });

        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let hashed_password = hash_password(&body.password.to_string());

    let query_result = sqlx::query_as!(
        AuthUserModel,
        "INSERT INTO auth_users (email,password) VALUES ($1, $2) RETURNING *",
        body.email.to_string(),
        hashed_password,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(auth_user) => {
            let auth_user_response = json!({"status": "success","data": json!({
                "auth_user": auth_user
            })});

            return Ok((StatusCode::CREATED, Json(auth_user_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "AuthUser with that email already exists",
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
