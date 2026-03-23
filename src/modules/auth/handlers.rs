use super::models::OAuthCallbackQuery;
use super::services::AuthService;
use crate::{core::extractors::AuthUser, error::AppError, modules::usersv2::models::UserResponse};
use axum::{Json, extract::Query, response::Redirect};
use axum_extra::extract::CookieJar;

pub async fn login_handler(
    service: AuthService,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AppError> {
    service.login(jar).await
}

pub async fn logout_handler() {}

pub async fn callback_handler(
    service: AuthService,
    jar: CookieJar,
    Query(query): Query<OAuthCallbackQuery>,
) -> Result<(CookieJar, Redirect), AppError> {
    service.callback(jar, query).await
}

pub async fn me_handler(
    service: AuthService,
    AuthUser(user): AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    service.me(user).await
}
