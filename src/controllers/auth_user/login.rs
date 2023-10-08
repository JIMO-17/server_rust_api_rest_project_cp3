use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{
    models::auth_user::AuthUserModel, schema::auth_user::CreateAuthUsetSchema,
    utils::fn_password::is_valid_password, AppState,
};

pub async fn login_handler(
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

    let query_result = sqlx::query_as!(
        AuthUserModel,
        "SELECT * FROM auth_users WHERE email = $1",
        body.email.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(auth_user) => {
            // println!("entre");
            // println!("{}", serde_json::json!(auth_user));

            let valid = is_valid_password(&auth_user.password, &body.password).await;

            if valid {
                let auth_user_response = json!({"status": "success","data": json!({
                    "auth_user": auth_user
                })});

                return Ok((StatusCode::OK, Json(auth_user_response)));
            } else {
                let error_response = serde_json::json!({
                    "status": "failed",
                    "message": "Invalid password",
                });

                return Err((StatusCode::BAD_REQUEST, Json(error_response)));
            }
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
