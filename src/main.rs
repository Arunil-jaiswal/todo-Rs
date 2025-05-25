mod handler;
mod model;
mod schema;

use std::sync::Arc;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};



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
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    println!("Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}