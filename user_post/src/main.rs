use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::database::DB::DB;
use crate::handlers::UserHandler;
use crate::models::User::UserFollow;

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
        .route("/register", post(UserHandler::register))
        .route("/login", post(UserHandler::login))
        .route("/user/follow/{followed_id}", post(UserHandler::add_follower).delete(UserHandler::remove_follower))
        .route("/user/", get(UserHandler::get_profile))
        .route("/user/post", post(UserHandler::write_post))
        .route("/user/post/{id}", get(UserHandler::get_post))
        .with_state(db);
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("User Post Server Started!");

    axum::serve(listener, app).await.unwrap();
}
