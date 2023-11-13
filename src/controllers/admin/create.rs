use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{
    models::admin::AdminModel, schema::admin::CreateAdminSchema, AppState,
};

pub async fn create_admin_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateAdminSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        AdminModel,
        "INSERT INTO admins (identification,identification_type,name,last_name,phonenumber,address,auth_user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        body.identification.to_string(),
        body.identification_type.to_string(),
        body.name.to_string(),
        body.last_name,
        body.phonenumber.to_string(),
        body.address,
        body.auth_user_id,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(admin) => {
            let admin_response = json!({"status": "success","data": json!({
                "admin": admin
            })});

            return Ok((StatusCode::CREATED, Json(admin)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "admin already exists",
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
