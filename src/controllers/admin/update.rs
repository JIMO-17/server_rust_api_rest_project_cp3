use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::admin::AdminModel, schema::admin::UpdateAdminSchema, AppState};

// update auth_user
pub async fn update_admin_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateAuthUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(AuthUserModel, "SELECT * FROM admins WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("admin not found")
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let auth_user = query_result.unwrap();

    let query_result = sqlx::query_as!(
        AdminModel,
        "UPDATE admins SET identification = $1, identification_type = $2, name = $3, last_name = $4, phonenumber = $5, address = $6, auth_user_id = $7, updated_at = $8 WHERE id = $9 RETURNING *",
        body.identification.to_string(),
        body.identification_type.to_string(),
        body.name.to_string(),
        body.last_name.to_string(),
        body.phonenumber.to_string(),
        body.address.to_string(),
        body.auth_user_id.to_string(),
        now,
        id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(admin) => {
            let admin_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "admin": admin
            })});

            return Ok(Json(admin_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
