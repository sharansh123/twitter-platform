use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use crate::utils::SecurityUtil;
use crate::utils::SecurityUtil::Claims;

impl<S> FromRequestParts<S> for Claims where S: Send + Sync {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let token_data = SecurityUtil::decode_token(bearer.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(token_data)
    }
}