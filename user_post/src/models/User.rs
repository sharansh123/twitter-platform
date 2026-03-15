use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserFollow{
    pub follower_id: String,
    pub followed_id: String
}

#[derive(Deserialize, Serialize)]
pub struct UserProfile{
    pub id: String,
    pub name: String,
    pub followers: i32,
    pub following: i32
}

#[derive(Deserialize, Serialize)]
pub struct UserPost {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub user_id: String,
    pub content: String,
    #[serde(default)]
    pub created_at: NaiveDateTime,
}