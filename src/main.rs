use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod controllers;
mod error;
mod models;
mod schema;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Crud rest api to electiva CP3";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the dbis successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the db: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .with_state(app_state);

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
