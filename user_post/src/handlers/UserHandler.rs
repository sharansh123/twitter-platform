use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::DB::DB;
use crate::models::Auth::{Claims, Token, UserAuth};
use crate::models::User::{UserFollow, UserProfile};

pub async fn register(State(db): State<Arc<DB>>, Json(user_info): Json<UserAuth>) -> Result<Json<Token>, StatusCode> {
    db.register(user_info).await.map(Json)
}

pub async fn login(State(db): State<Arc<DB>>, Json(user_info): Json<UserAuth>) -> Result<Json<Token>, StatusCode> {
    db.login(user_info).await.map(Json)
}

pub async fn add_follower(claims: Claims, State(db): State<Arc<DB>>, Path(followed_id): Path<String>) -> Result<(), StatusCode> {
    let user_follow = UserFollow{
        follower_id: claims.sub,
        followed_id,
    };
    db.add_followers(user_follow).await
}

pub async fn remove_follower(claims: Claims, State(db): State<Arc<DB>>, Path(followed_id): Path<String>) -> Result<(), StatusCode> {
    let user_follow = UserFollow{
        follower_id: claims.sub,
        followed_id,
    };
    db.remove_followers(user_follow).await
}
pub async fn get_profile(claims: Claims, State(db): State<Arc<DB>>) -> Result<Json<UserProfile>, StatusCode> {
    db.user_profile(claims.sub).await.map(Json)
}