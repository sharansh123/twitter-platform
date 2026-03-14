use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::DB::DB;
use crate::models::Auth::{Token, UserAuth};


pub async fn register(State(db): State<Arc<DB>>, Json(user_info): Json<UserAuth>) -> Result<Json<Token>, StatusCode> {
    db.register(user_info).await.map(Json)
}

pub async fn login(State(db): State<Arc<DB>>, Json(user_info): Json<UserAuth>) -> Result<Json<Token>, StatusCode> {
    db.login(user_info).await.map(Json)
}