use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{models::auth_user::AuthUserModel, schema::auth_user::FilterOptions, AppState};

pub async fn auth_user_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        AuthUserModel,
        "SELECT * FROM auth_users ORDER BY id DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "Failed to fetch auth_users"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let auth_users = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": auth_users.len(),
        "auth_users": auth_users
    });

    return Ok(Json(json_response));
}
