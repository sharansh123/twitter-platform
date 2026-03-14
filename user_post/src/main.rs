use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use crate::database::DB::DB;

mod database;
mod handlers;
mod models;
mod utils;
mod security;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = Arc::new(DB::new().await);
    let app: Router = Router::new().route("/health", get(async || "Up and Running".to_string()))
        .with_state(db);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("User Post Server Started!");

    axum::serve(listener, app).await.unwrap();
}
