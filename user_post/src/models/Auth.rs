use serde::{Deserialize, Serialize};

pub type JWTError = jsonwebtoken::errors::Error;
#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
    pub token_type: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims{
    pub sub: String,
    pub exp: usize
}

#[derive(Deserialize, Serialize)]
pub struct UserAuth{
    pub user_id: String,
    pub name: String,
    pub password: String
}

