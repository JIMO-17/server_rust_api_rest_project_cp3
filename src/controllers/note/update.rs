use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::note::NoteModel, schema::note::UpdateNoteSchema, AppState};

// update note
pub async fn edit_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
