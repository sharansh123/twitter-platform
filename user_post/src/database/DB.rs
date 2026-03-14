use argon2::PasswordHash;
use axum::http::StatusCode;
use chrono::Duration;
use sqlx::postgres::PgPoolOptions;
use crate::models::Auth::{Token, UserAuth};
use crate::models::User::{UserFollow, UserProfile};
use crate::utils::SecurityUtil;
use crate::utils::SecurityUtil::generate_token;

pub struct DB {
    db: sqlx::PgPool
}

impl DB {
    pub async fn new() -> Self {
        let url = std::env::var("DATABASE_URL").expect("Database not found");
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await
            .expect("Failed to connect to DB");
        
        DB {
            db: pool
        }
    }

    pub async fn register(&self, user_auth: UserAuth) -> Result<Token, StatusCode> {

        let password_hash = SecurityUtil::generate_hash(&user_auth.password).map_err(|_| StatusCode::UNAUTHORIZED)?;

        sqlx::query!("INSERT INTO users VALUES ($1, $2, $3, 0, 0, NOW(), NOW())",
            user_auth.user_id,
            user_auth.name,
            password_hash
        ).execute(&self.db)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

        generate_token(user_auth.user_id, Duration::minutes(60)).map_err(|_| StatusCode::UNAUTHORIZED)

    }


    pub async fn login(&self, user_auth: UserAuth) -> Result<Token, StatusCode> {

        let fetched_hash = sqlx::query_scalar!(
            "select password_hash from users where id = $1", user_auth.user_id
        ).fetch_one(&self.db)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        let password_hash = PasswordHash::new(&fetched_hash).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
        SecurityUtil::verify(&user_auth.password, password_hash)?;

        generate_token(user_auth.user_id, Duration::minutes(60)).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)
    }

    pub async fn add_followers(&self, user_follow: UserFollow) -> Result<(), StatusCode> {
        if user_follow.follower_id == user_follow.followed_id {
            return Err(StatusCode::FORBIDDEN)
        }

        sqlx::query!("INSERT INTO user_follow VALUES ($1, $2)", user_follow.follower_id, user_follow.followed_id)
            .execute(&self.db)
            .await
            .map_err(|_| StatusCode::FORBIDDEN)?;

        Ok(())
    }

    pub async fn remove_followers(&self, user_follow: UserFollow) -> Result<(), StatusCode> {

        if user_follow.follower_id == user_follow.followed_id {
            return Err(StatusCode::FORBIDDEN)
        }

        sqlx::query!("DELETE FROM user_follow WHERE follower_id = $1 and followed_id = $2", user_follow.follower_id, user_follow.followed_id)
            .execute(&self.db)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        Ok(())
    }

    pub async fn user_profile(&self, user_id: String) -> Result<UserProfile, StatusCode>{
        sqlx::query_as!(
            UserProfile,
            "select id, name, followers, following from users where id = $1",
            user_id
        ).fetch_one(&self.db)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)
    }

}