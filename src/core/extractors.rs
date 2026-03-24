use crate::error::AppError;
use crate::modules::auth::models::JwtClaims;
use crate::modules::auth::services::{AuthService, JwtService};
use crate::modules::meals::services::MealService;
use crate::modules::profiles::services::ProfileService;
use crate::modules::users::services::UserService;
use crate::state::AppState;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;

pub struct AuthUser(pub JwtClaims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt_service =
            JwtService::new(state.config.jwt_secret.clone(), state.config.jwt_expiration);
        let jar = CookieJar::from_request_parts(parts, &state)
            .await
            .map_err(|e| {
                tracing::error!("Failed to extract cookies: {}", e);
                AppError::Unauthorized(Some("Failed to extract cookies".to_string()))
            })?;

        let token = jar
            .get("access_token")
            .ok_or(AppError::Unauthorized(Some(
                "Missing access token".to_string(),
            )))
            .map(|cookie| cookie.value().to_string())
            .map_err(|_| AppError::Unauthorized(Some("Missing access token".to_string())))?;

        let claims = jwt_service.decode(&token)?;

        Ok(AuthUser(claims))
    }
}

impl FromRequestParts<AppState> for AuthService {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(AuthService::new(state.clone()))
    }
}

impl FromRequestParts<AppState> for UserService {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(UserService::new(state.clone()))
    }
}

impl FromRequestParts<AppState> for ProfileService {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(ProfileService::new(state.clone()))
    }
}

impl FromRequestParts<AppState> for MealService {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(MealService::new(state.clone()))
    }
}
