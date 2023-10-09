use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controllers::{
        auth_user::{
            auth_user_list_handler, create_auth_user_handler, delete_auth_user_handler,
            get_auth_user_by_access_token_handler, get_auth_user_handler, login_handler,
            update_auth_user_handler,
        },
        health_checker::health_checker_handler,
        note::{
            create_note_handler, delete_note_handler, edit_note_handler, get_note_handler,
            note_list_handler,
        },
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/notes", get(note_list_handler))
        .route("/api/notes/", post(create_note_handler))
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .route("/api/auth_users", get(auth_user_list_handler))
        .route("/api/auth_user/", post(create_auth_user_handler))
        .route("/api/auth_user/login", post(login_handler))
        .route(
            "/api/auth_user/:id",
            get(get_auth_user_handler)
                .patch(update_auth_user_handler)
                .delete(delete_auth_user_handler),
        )
        .route(
            "/api/auth_user/access_token/:access_token",
            get(get_auth_user_by_access_token_handler),
        )
        .with_state(app_state)
}
