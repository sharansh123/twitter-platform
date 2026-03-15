use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::DB::DB;
use crate::database::KafkaConfig::Message;
use crate::models::Auth::{Claims, Token, UserAuth};
use crate::models::User::{UserFollow, UserPost, UserProfile};

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

pub async fn write_post(claims: Claims, State(db): State<Arc<DB>>, Json(user_post): Json<UserPost>) -> Result<Json<UserPost>, StatusCode> {
    let result = db.user_post(user_post, claims.sub).await.map(Json)?;
    let topic = std::env::var("KAFKA_TOPIC").expect("Brokers not found");
    let message = Message{
        post_id: result.id.clone(),
        user_id: result.user_id.clone()
    };
    db.producer.send(topic, message).await;
    Ok(result)
}

pub async fn get_post(claims: Claims, State(db): State<Arc<DB>>, Path(id): Path<i32>) -> Result<Json<UserPost>, StatusCode> {
    db.get_post(id).await.map(Json)
}