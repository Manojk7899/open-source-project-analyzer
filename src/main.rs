mod handlers;
mod models;
mod routes;
mod state;
mod scoring;
mod ai_handlers;
mod ai_models;
mod report_models;
mod report_handlers;


use axum::{routing::get,
     Json, Router,
    http::{
        header::CONTENT_TYPE,
        Method,
    },};
    use tower_http::cors::{Any, CorsLayer};
use serde::Serialize;

use crate::routes::repository_routes;
use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;    
use crate::state::AppState;

#[derive(Clone)]

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    //databse_url defined in .env file 
    let database_url=
        env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let pool=PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url).await
            .expect("failed to connect to the database ");
            sqlx::migrate!()
            .run(&pool).await
            .expect("failed to run Migrations");
        let gemini_model=
        env::var("GEMINI_MODEL") 
            .unwrap_or_else(|_| "gemini-flash-latest".to_string());
    let state:AppState =AppState{
        db:pool,
        gemini_model, 
    };
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
        ])
        .allow_headers([CONTENT_TYPE]);
      let app = Router::new()
        .route("/health", get(health))
        .nest("/api/repository",
         repository_routes()
        )
        .layer(cors) 
            .with_state(state); 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("failed to bind the address");

    println!("server running on http://127.0.0.1:8081/health");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}