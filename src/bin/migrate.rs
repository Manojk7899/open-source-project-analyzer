use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to PostgreSQL");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    println!("Database migrations completed");
}