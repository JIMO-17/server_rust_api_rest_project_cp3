use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{error::AppError, models};

pub async fn register(
    Json(credentials): Json<models::auth::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, AppError> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        // return Err(AppError::BadRequest("Email and password are required".to_string()));
        return Err(AppError::MissingCredential);
    }

    let user = sqlx::query_as::<_, models::auth::User>(
        "SELECT id, email, password FROM users WHERE email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        AppError::InternalServerError
    })?;

    if let Some(_) = user {
        // return Err(AppError::BadRequest(
        //     "A user with that email already exists".to_string(),
        // ));
        return Err(AppError::UserAlreadyExists);
    }

    let result = sqlx::query("INSERT INTO users (id, email, password) VALUES ($1, $2, $3)")
        .bind(&uuid::Uuid::new_v4().to_string())
        .bind(&credentials.email)
        .bind(credentials.password)
        .execute(&pool)
        .await
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })?;

    if result.rows_affected() < 1 {
        return Err(AppError::InternalServerError);
    } else {
        Ok(Json(json!({ "msg": "registered successfully" })))
    }
}