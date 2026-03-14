use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, SaltString};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::models::Auth::{Claims, JWTError, Token};

pub fn generate_hash(code: &String) -> Result<String, Error> {
    let argon = Argon2::default();
    let random_salt = SaltString::generate(&mut OsRng);
    argon.hash_password(code.as_bytes(), &random_salt).map(|x| x.to_string())
}

pub fn generate_token(user_id: String, duration: Duration) -> Result<Token, JWTError> {
    let expiration = Utc::now()
        .checked_add_signed(duration)
        .expect("Invalid Expiration")
        .timestamp() as usize;
    
    let claims = Claims{
        sub: user_id,
        exp: expiration
    };
    let secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT Secret not found");
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes())
    );
    
    token.map(|x| Token{
        token: x,
        token_type: String::from("Bearer")
    })
}

pub fn decode_token(token: &str) -> Result<Claims, JWTError> {
    
    let secret_key = std::env::var("JWT_SECRET_KEY").expect("JWT Secret not found");
    
    decode::<Claims>(token, 
    &DecodingKey::from_secret(secret_key.as_bytes()),
    &Validation::default()
    ).map(|x| x.claims)
}

pub fn verify(password: &String, password_hash: PasswordHash) -> Result<(), StatusCode> {
    Argon2::default().verify_password(password.as_bytes(), &password_hash).map_err(|x| StatusCode::UNAUTHORIZED) 
}